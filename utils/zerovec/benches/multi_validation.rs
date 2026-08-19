// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Validation performance benchmarks comparing existing/baseline ULE validation
//! against potential optimized implementations across 10, 100, and 1,000 elements.

use core::mem::size_of;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use tinystr::{TinyAsciiStr, UnvalidatedTinyAsciiStr};
use zerovec::ule::tuple::Tuple2ULE;
use zerovec::ule::*;
use zerovec::{VarZeroVec, ZeroSlice, ZeroVec};

const COUNTS: [usize; 3] = [10, 100, 1000];

// =========================================================================
// 1. All-POD Struct: derive(ULE) vs Manual O(1) Length Check
//
// Tests whether derive(ULE) on an all-POD struct collapses to O(1) at compile
// time, compared to an explicit single modulo length check.
// =========================================================================

#[zerovec::make_ule(PointULE)]
#[zerovec::derive(Debug)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point {
    x: u32,
    y: u32,
    z: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct OptimizedPointULE(PointULE);

unsafe impl ULE for OptimizedPointULE {
    #[inline]
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        if bytes.len() % size_of::<PointULE>() != 0 {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        Ok(())
    }
}

fn bench_pod_struct(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_validation/1_pod_struct");
    for count in COUNTS {
        let points = vec![Point { x: 1, y: 2, z: 3 }; count];
        let bytes = ZeroVec::<Point>::alloc_from_slice(&points)
            .as_bytes()
            .to_vec();

        group.bench_with_input(
            BenchmarkId::new("baseline_derive_make_ule", count),
            &bytes,
            |b, bytes| {
                b.iter(|| ZeroSlice::<Point>::parse_bytes(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_manual_o1", count),
            &bytes,
            |b, bytes| {
                b.iter(|| OptimizedPointULE::validate_bytes(black_box(bytes)).unwrap());
            },
        );
    }
    group.finish();
}

// =========================================================================
// 2. Tuple POD: chunks (un-collapsed O(N)) vs chunks_exact (collapsed O(1))
//
// Demonstrates the impact of using chunks_exact instead of chunks for tuple
// validation, allowing LLVM to statically prove in-bounds slice indexing.
// =========================================================================

#[inline]
fn validate_tuple_unoptimized_chunks(bytes: &[u8]) -> Result<(), UleError> {
    const SIZE: usize = 8;
    if bytes.len() % SIZE != 0 {
        return Err(UleError::length::<Tuple2ULE<RawBytesULE<4>, RawBytesULE<4>>>(bytes.len()));
    }
    for chunk in bytes.chunks(SIZE) {
        #[expect(clippy::indexing_slicing)]
        <RawBytesULE<4>>::validate_bytes(&chunk[0..4])?;
        #[expect(clippy::indexing_slicing)]
        <RawBytesULE<4>>::validate_bytes(&chunk[4..8])?;
    }
    Ok(())
}

fn bench_tuple_pod(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_validation/2_tuple_pod");
    for count in COUNTS {
        let u32_pairs = vec![(1234u32, 5678u32); count];
        let bytes = ZeroVec::<(u32, u32)>::alloc_from_slice(&u32_pairs)
            .as_bytes()
            .to_vec();

        group.bench_with_input(
            BenchmarkId::new("baseline_chunks_uncollapsed", count),
            &bytes,
            |b, bytes| {
                b.iter(|| validate_tuple_unoptimized_chunks(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_chunks_exact_collapsed", count),
            &bytes,
            |b, bytes| {
                b.iter(|| ZeroSlice::<(u32, u32)>::parse_bytes(black_box(bytes)).unwrap());
            },
        );
    }
    group.finish();
}

// =========================================================================
// 3. OptionULE: byte-by-byte iterator vs 32-bit word zero check
//
// Compares OptionULE's baseline byte-by-byte padding check (`.iter().all(== 0)`)
// against a word-at-a-time (`u32::from_ne_bytes(...) == 0`) check.
// =========================================================================

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct OptimizedOptionU32ULE(OptionULE<RawBytesULE<4>>);

unsafe impl ULE for OptimizedOptionU32ULE {
    #[inline]
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        const SIZE: usize = 5;
        if bytes.len() % SIZE != 0 {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        for chunk in bytes.chunks_exact(SIZE) {
            #[expect(clippy::indexing_slicing)]
            match chunk[0] {
                0 => {
                    let word = [chunk[1], chunk[2], chunk[3], chunk[4]];
                    if u32::from_ne_bytes(word) != 0 {
                        return Err(UleError::parse::<Self>());
                    }
                }
                1 => <RawBytesULE<4>>::validate_bytes(&chunk[1..])?,
                _ => return Err(UleError::parse::<Self>()),
            }
        }
        Ok(())
    }
}

fn bench_option_none(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_validation/3_option_none");
    for count in COUNTS {
        let none_values = vec![None::<u32>; count];
        let bytes = ZeroVec::<Option<u32>>::alloc_from_slice(&none_values)
            .as_bytes()
            .to_vec();

        group.bench_with_input(
            BenchmarkId::new("baseline_byte_iter_all", count),
            &bytes,
            |b, bytes| {
                b.iter(|| ZeroSlice::<Option<u32>>::parse_bytes(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_word_check_u32", count),
            &bytes,
            |b, bytes| {
                b.iter(|| OptimizedOptionU32ULE::validate_bytes(black_box(bytes)).unwrap());
            },
        );
    }
    group.finish();
}

// =========================================================================
// 4. ASCII Strings: byte-by-byte vs 64-bit SWAR bitmasking
//
// Compares TinyAsciiStr's standard per-byte range validation against 64-bit
// SWAR (SIMD within a register) word masking.
// =========================================================================

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(transparent)]
struct OptimizedTinyAsciiStr4(TinyAsciiStr<4>);

unsafe impl ULE for OptimizedTinyAsciiStr4 {
    #[inline]
    fn validate_bytes(bytes: &[u8]) -> Result<(), UleError> {
        const N: usize = 4;
        if bytes.len() % N != 0 {
            return Err(UleError::length::<Self>(bytes.len()));
        }
        let (prefix, u64_chunks, suffix) = unsafe { bytes.align_to::<u64>() };
        for &b in prefix {
            if b == 0 || b >= 128 {
                return Err(UleError::parse::<Self>());
            }
        }
        for &word in u64_chunks {
            let has_high_bit = (word & 0x8080808080808080) != 0;
            let has_zero =
                ((word.wrapping_sub(0x0101010101010101)) & !word & 0x8080808080808080) != 0;
            if has_high_bit || has_zero {
                return Err(UleError::parse::<Self>());
            }
        }
        for &b in suffix {
            if b == 0 || b >= 128 {
                return Err(UleError::parse::<Self>());
            }
        }
        Ok(())
    }
}

fn bench_ascii_tinystr(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_validation/4_ascii_tinystr");
    for count in COUNTS {
        let strings: Vec<TinyAsciiStr<4>> = (0..count)
            .map(|i| {
                let s = format!("{:04}", i % 10000);
                s.parse().unwrap()
            })
            .collect();
        let bytes = ZeroVec::<TinyAsciiStr<4>>::alloc_from_slice(&strings)
            .as_bytes()
            .to_vec();

        group.bench_with_input(
            BenchmarkId::new("baseline_byte_by_byte", count),
            &bytes,
            |b, bytes| {
                b.iter(|| ZeroSlice::<TinyAsciiStr<4>>::parse_bytes(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized_swar_u64", count),
            &bytes,
            |b, bytes| {
                b.iter(|| OptimizedTinyAsciiStr4::validate_bytes(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unvalidated_o1", count),
            &bytes,
            |b, bytes| {
                b.iter(|| {
                    ZeroSlice::<UnvalidatedTinyAsciiStr<4>>::parse_bytes(black_box(bytes)).unwrap()
                });
            },
        );
    }
    group.finish();
}

// =========================================================================
// 5. VarZeroVec: Element Loop vs Bulk Contiguous Payload Validation
//
// Compares per-element parsing loops against single bulk payload validation
// (e.g. SIMD-backed bulk UTF-8 validation) combined with index boundary checks.
// =========================================================================

#[inline]
fn validate_vzv_bytes_bulk(bytes: &[u8]) -> Result<(), UleError> {
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.len() < 2 {
        return Err(UleError::parse::<ZeroSlice<u8>>());
    }
    let len = u16::from_le_bytes([bytes[0], bytes[1]]) as usize;
    if len == 0 {
        return if bytes.len() == 2 {
            Ok(())
        } else {
            Err(UleError::parse::<ZeroSlice<u8>>())
        };
    }
    let indices_len = (len - 1) * 2;
    if bytes.len() < 2 + indices_len {
        return Err(UleError::parse::<ZeroSlice<u8>>());
    }
    let indices_bytes = &bytes[2..2 + indices_len];
    let things_len = bytes.len() - (2 + indices_len);
    let mut prev = 0;
    for chunk in indices_bytes.chunks_exact(2) {
        let idx = u16::from_le_bytes([chunk[0], chunk[1]]) as usize;
        if idx < prev || idx > things_len {
            return Err(UleError::parse::<ZeroSlice<u8>>());
        }
        prev = idx;
    }
    Ok(())
}

#[inline]
fn validate_vzv_str_bulk(bytes: &[u8]) -> Result<(), UleError> {
    if bytes.is_empty() {
        return Ok(());
    }
    if bytes.len() < 2 {
        return Err(UleError::parse::<str>());
    }
    let len = u16::from_le_bytes([bytes[0], bytes[1]]) as usize;
    if len == 0 {
        return if bytes.len() == 2 {
            Ok(())
        } else {
            Err(UleError::parse::<str>())
        };
    }
    let indices_len = (len - 1) * 2;
    if bytes.len() < 2 + indices_len {
        return Err(UleError::parse::<str>());
    }
    let indices_bytes = &bytes[2..2 + indices_len];
    let things = &bytes[2 + indices_len..];

    // 1. Validate the entire contiguous string payload in a single SIMD-backed call
    core::str::from_utf8(things).map_err(|_| UleError::parse::<str>())?;

    // 2. Validate index monotonicity and that slice indices do not split multi-byte UTF-8 sequences
    let mut prev = 0;
    for chunk in indices_bytes.chunks_exact(2) {
        let idx = u16::from_le_bytes([chunk[0], chunk[1]]) as usize;
        if idx < prev || idx > things.len() {
            return Err(UleError::parse::<str>());
        }
        if idx < things.len() && (things[idx] & 0xC0 == 0x80) {
            return Err(UleError::parse::<str>());
        }
        prev = idx;
    }
    Ok(())
}

fn bench_varzerovec_bulk(c: &mut Criterion) {
    let mut group = c.benchmark_group("multi_validation/5_varzerovec");
    for count in COUNTS {
        // VarZeroVec<[u8]>
        let byte_slices: Vec<Vec<u8>> = (0..count).map(|i| vec![(i % 256) as u8; 8]).collect();
        let byte_slice_refs: Vec<&[u8]> = byte_slices.iter().map(|s| s.as_slice()).collect();
        let bytes_buf = VarZeroVec::<[u8]>::from(&byte_slice_refs)
            .as_bytes()
            .to_vec();

        group.bench_with_input(
            BenchmarkId::new("bytes_baseline_per_element", count),
            &bytes_buf,
            |b, bytes| {
                b.iter(|| VarZeroVec::<[u8]>::parse_bytes(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("bytes_optimized_bulk_indices", count),
            &bytes_buf,
            |b, bytes| {
                b.iter(|| validate_vzv_bytes_bulk(black_box(bytes)).unwrap());
            },
        );

        // VarZeroVec<str>
        let string_slices: Vec<String> = (0..count)
            .map(|i| format!("hello_unicode_world_{:04}", i))
            .collect();
        let string_slice_refs: Vec<&str> = string_slices.iter().map(|s| s.as_str()).collect();
        let str_buf = VarZeroVec::<str>::from(&string_slice_refs)
            .as_bytes()
            .to_vec();

        group.bench_with_input(
            BenchmarkId::new("str_baseline_per_element", count),
            &str_buf,
            |b, bytes| {
                b.iter(|| VarZeroVec::<str>::parse_bytes(black_box(bytes)).unwrap());
            },
        );

        group.bench_with_input(
            BenchmarkId::new("str_optimized_bulk_utf8", count),
            &str_buf,
            |b, bytes| {
                b.iter(|| validate_vzv_str_bulk(black_box(bytes)).unwrap());
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_pod_struct,
    bench_tuple_pod,
    bench_option_none,
    bench_ascii_tinystr,
    bench_varzerovec_bulk,
);
criterion_main!(benches);
