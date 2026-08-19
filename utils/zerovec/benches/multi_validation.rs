// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Benchmarks comparing baseline `ULE`/`VarULE` slice validation against
//! hand-optimized `Optimized<T>` implementations across 10, 100, and 1,000 elements.

use core::mem::size_of;
use core::num::NonZeroU8;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tinystr::{tinystr, TinyAsciiStr};
use zerovec::ule::tuple::Tuple2ULE;
use zerovec::ule::*;
use zerovec::{VarZeroSlice, VarZeroVec, ZeroSlice, ZeroVec};

const COUNTS: [usize; 3] = [10, 100, 1000];

/// Transparent wrapper used to attach an alternative/optimized `ULE` or `VarULE`
/// implementation to an existing type `T` for side-by-side comparison.
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Optimized<T: ?Sized>(T);

impl<T: AsULE> AsULE for Optimized<T>
where
    Optimized<T::ULE>: ULE,
{
    type ULE = Optimized<T::ULE>;
    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        Optimized(self.0.to_unaligned())
    }
    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Optimized(T::from_unaligned(unaligned.0))
    }
}

fn bench_slice<T>(c: &mut Criterion, name: &str, item: T)
where
    T: AsULE + Copy,
    Optimized<T::ULE>: ULE,
{
    let mut group = c.benchmark_group(name);
    for count in COUNTS {
        let vec = ZeroVec::<T>::alloc_from_slice(&vec![item; count]);
        let bytes = vec.as_bytes();
        group.bench_with_input(BenchmarkId::new("baseline", count), bytes, |b, bytes| {
            b.iter(|| ZeroSlice::<T>::parse_bytes(black_box(bytes)).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("optimized", count), bytes, |b, bytes| {
            b.iter(|| ZeroSlice::<Optimized<T>>::parse_bytes(black_box(bytes)).unwrap());
        });
    }
    group.finish();
}

fn bench_var<T>(c: &mut Criterion, name: &str, item: &T)
where
    T: VarULE + ?Sized + 'static,
    Optimized<VarZeroSlice<T>>: VarULE,
{
    let mut group = c.benchmark_group(name);
    for count in COUNTS {
        let vec = VarZeroVec::<T>::from(&vec![item; count]);
        let bytes = vec.as_bytes();
        group.bench_with_input(BenchmarkId::new("baseline", count), bytes, |b, bytes| {
            b.iter(|| VarZeroSlice::<T>::parse_bytes(black_box(bytes)).unwrap());
        });
        group.bench_with_input(BenchmarkId::new("optimized", count), bytes, |b, bytes| {
            b.iter(|| Optimized::<VarZeroSlice<T>>::parse_bytes(black_box(bytes)).unwrap());
        });
    }
    group.finish();
}

// ============================================================================
// 1–3. Infallible Composite Types: O(1) Length Check vs Chunk Loop
//
// Tests whether an explicit `IS_INFALLIBLE` O(1) modulo length check beats
// the `chunks_exact` loop emitted by `#[make_ule]`, `Tuple2ULE`, and `NichedOptionULE`.
// ============================================================================

macro_rules! impl_infallible_ule {
    ($ule:ty) => {
        unsafe impl ULE for Optimized<$ule> {
            #[inline]
            fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
                #[expect(clippy::modulo_one)]
                if bytes.len() % size_of::<Self>() == 0 {
                    Ok(())
                } else {
                    Err(UleError::length::<Self>(bytes.len()))
                }
            }
        }
    };
}

#[zerovec::make_ule(PointULE)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

impl_infallible_ule!(PointULE);
impl_infallible_ule!(Tuple2ULE<RawBytesULE<4>, RawBytesULE<4>>);
impl_infallible_ule!(NichedOptionULE<NonZeroU8, 1>);

// ============================================================================
// 4. OptionULE<U>: Whole-Chunk Pattern Match vs `.iter().all(|x| *x == 0)`
// ============================================================================

unsafe impl ULE for Optimized<OptionULE<RawBytesULE<4>>> {
    #[inline]
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        if bytes.len() % size_of::<Self>() != 0 {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        for chunk in bytes.chunks_exact(size_of::<Self>()) {
            match chunk {
                [0, 0, 0, 0, 0] => {}
                [1, rest @ ..] => RawBytesULE::<4>::validate_bytes(rest)?,
                _ => return Err(UleError::parse::<Self>()),
            }
        }
        Ok(())
    }
}

// ============================================================================
// 5. TinyAsciiStr<4>: u32 Word Fast Path vs Per-Byte Construction Loop
// ============================================================================

unsafe impl ULE for Optimized<TinyAsciiStr<4>> {
    #[inline]
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        if bytes.len() % 4 != 0 {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        for chunk in bytes.chunks_exact(4) {
            let raw: [u8; 4] = chunk.try_into().unwrap();
            let word = u32::from_ne_bytes(raw);
            // Fast path: all 4 bytes are non-null ASCII (0x01..=0x7F)
            if (word & 0x8080_8080) == 0
                && (word.wrapping_sub(0x0101_0101) & !word & 0x8080_8080) == 0
            {
                continue;
            }
            TinyAsciiStr::<4>::try_from_raw(raw).map_err(|_| UleError::parse::<Self>())?;
        }
        Ok(())
    }
}

// ============================================================================
// 6. VarZeroVec: Bulk Contiguous Payload Validation vs Per-Element Loop
// ============================================================================

#[inline]
fn parse_vzv_header(bytes: &[u8]) -> Result<(&[RawBytesULE<2>], &[u8]), UleError> {
    let Some((&len_bytes, rest)) = bytes.split_first_chunk::<2>() else {
        return if bytes.is_empty() {
            Ok((&[], &[]))
        } else {
            Err(UleError::parse::<VarZeroSlice<[u8]>>())
        };
    };
    let len = u16::from_le_bytes(len_bytes) as usize;
    let Some((indices_bytes, things)) = rest.split_at_checked(len.saturating_sub(1) * 2) else {
        return Err(UleError::parse::<VarZeroSlice<[u8]>>());
    };
    if len == 0 && !things.is_empty() {
        return Err(UleError::parse::<VarZeroSlice<[u8]>>());
    }
    let indices = unsafe { RawBytesULE::<2>::slice_from_bytes_unchecked(indices_bytes) };
    let mut prev = 0;
    for idx in indices {
        let idx = idx.as_unsigned_int() as usize;
        if idx < prev {
            return Err(UleError::parse::<VarZeroSlice<[u8]>>());
        }
        prev = idx;
    }
    if prev > things.len() {
        return Err(UleError::parse::<VarZeroSlice<[u8]>>());
    }
    Ok((indices, things))
}

macro_rules! impl_vzv_varule {
    ($inner:ty, |$indices:ident, $things:ident| $body:expr) => {
        unsafe impl VarULE for Optimized<VarZeroSlice<$inner>> {
            #[inline]
            fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
                let ($indices, $things) = parse_vzv_header(bytes)?;
                $body
            }
            #[inline]
            unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
                &*(bytes as *const [u8] as *const Self)
            }
        }
    };
}

impl_vzv_varule!([u8], |_indices, _things| Ok(()));
impl_vzv_varule!(str, |indices, things| {
    let s = core::str::from_utf8(things).map_err(|_| UleError::parse::<Self>())?;
    if indices
        .iter()
        .any(|i| !s.is_char_boundary(i.as_unsigned_int() as usize))
    {
        return Err(UleError::parse::<Self>());
    }
    Ok(())
});

fn multi_validation_benches(c: &mut Criterion) {
    bench_slice(c, "multi_validation/pod_struct", Point { x: 1, y: 2, z: 3 });
    bench_slice(c, "multi_validation/pod_tuple", (1234u32, 5678u32));
    bench_slice(
        c,
        "multi_validation/niched_option",
        NichedOption(NonZeroU8::new(42)),
    );
    bench_slice(c, "multi_validation/option_none", None::<u32>);
    bench_slice(c, "multi_validation/option_some", Some(1234u32));
    bench_slice(c, "multi_validation/tinystr4", tinystr!(4, "enUS"));
    bench_var::<[u8]>(c, "multi_validation/vzv_bytes", &[1, 2, 3, 4, 5, 6, 7, 8]);
    bench_var::<str>(c, "multi_validation/vzv_str", "hello_unicode_world");
}

criterion_group!(benches, multi_validation_benches);
criterion_main!(benches);
