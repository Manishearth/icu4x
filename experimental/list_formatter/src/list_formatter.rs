// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use core::fmt::{self, Write};
use icu_locid::Locale;
use icu_provider::prelude::*;
use writeable::*;

/// Represents the type of a list. See the
/// [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns)
/// for an explanation of the different types.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ListType {
    /// A conjunction
    And,
    /// A disjunction
    Or,
    /// A list of units
    Unit,
}

/// Represents the style of a list. See the
/// [CLDR spec](https://unicode.org/reports/tr35/tr35-general.html#ListPatterns)
/// for an explanation of the different styles.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ListStyle {
    /// A typical list
    Wide,
    /// A shorter list
    Short,
    /// The shortest type of list
    Narrow,
}

/// A formatter that renders sequences of items in an i18n-friendly way. See the
/// [crate-level documentation](crate) for more details.
pub struct ListFormatter {
    data: DataPayload<ListFormatterPatternsV1Marker>,
    style: ListStyle,
}

impl ListFormatter {
    /// Creates a new [`ListFormatter`] from the given provider. The provider has to support the
    /// correct key for the given type:
    /// [`LIST_FORMAT_AND_V1`](crate::provider::key::LIST_FORMAT_AND_V1),
    /// [`LIST_FORMAT_OR_V1`](crate::provider::key::LIST_FORMAT_OR_V1), or
    /// [`LIST_FORMAT_UNIT_V1`](crate::provider::key::LIST_FORMAT_UNIT_V1).
    /// An error is returned if the key or language are not available, or if there were any
    /// deserialization errors.
    pub fn try_new<T: Into<Locale>, D: DataProvider<ListFormatterPatternsV1Marker> + ?Sized>(
        locale: T,
        data_provider: &D,
        type_: ListType,
        style: ListStyle,
    ) -> Result<Self, DataError> {
        let data = data_provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key: match type_ {
                        ListType::And => key::LIST_FORMAT_AND_V1,
                        ListType::Or => key::LIST_FORMAT_OR_V1,
                        ListType::Unit => key::LIST_FORMAT_UNIT_V1,
                    },
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(locale.into().into()),
                    },
                },
            })?
            .take_payload()?;
        Ok(Self { data, style })
    }

    /// Returns a [`Writeable`] composed of the input [`Writeable`]s and the language-dependent
    /// formatting. The first layer of parts contains [`parts::ELEMENT`] for input
    /// elements, and [`parts::LITERAL`] for list literals.
    pub fn format<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a>(
        &'a self,
        values: I,
    ) -> List<'a, W, I> {
        List {
            formatter: self,
            values,
        }
    }
}

/// The [`Part`]s used by [`ListFormatter`].
pub mod parts {
    use writeable::Part;

    /// The [`Part`] used by [`List`](super::List) to mark the part of the string that is an element.
    pub const ELEMENT: Part = Part {
        category: "list",
        value: "element",
    };

    /// The [`Part`] used by [`List`](super::List) to mark the part of the string that is a list literal,
    /// such as ", " or " and ".
    pub const LITERAL: Part = Part {
        category: "list",
        value: "literal",
    };
}

/// The [`Writeable`] implementation that is returned by [`ListFormatter::format`]. See
/// the [`writeable`] crate for how to consume this.
pub struct List<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a> {
    formatter: &'a ListFormatter,
    values: I,
}

impl<'a, W: Writeable + 'a, I: Iterator<Item = W> + Clone + 'a> Writeable for List<'a, W, I> {
    fn write_to_parts<V: PartsWrite + ?Sized>(&self, sink: &mut V) -> fmt::Result {
        macro_rules! literal {
            ($lit:ident) => {
                sink.with_part(parts::LITERAL, |l| l.write_str($lit))
            };
        }
        macro_rules! value {
            ($val:expr) => {
                sink.with_part(parts::ELEMENT, |e| $val.write_to_parts(e))
            };
        }

        let mut values = self.values.clone();

        if let Some(first) = values.next() {
            if let Some(second) = values.next() {
                if let Some(third) = values.next() {
                    // Start(values[0], middle(..., middle(values[n-3], End(values[n-2], values[n-1]))...)) =
                    // start_before + values[0] + start_between + (values[1..n-3] + middle_between)* +
                    // values[n-2] + end_between + values[n-1] + end_after

                    let (start_before, start_between, _) = self
                        .formatter
                        .data
                        .get()
                        .start(self.formatter.style)
                        .parts(&second);

                    literal!(start_before)?;
                    value!(first)?;
                    literal!(start_between)?;
                    value!(second)?;

                    let mut next = third;

                    for next_next in values {
                        let (_, between, _) = self
                            .formatter
                            .data
                            .get()
                            .middle(self.formatter.style)
                            .parts(&next);
                        literal!(between)?;
                        value!(next)?;
                        next = next_next;
                    }

                    let (_, end_between, end_after) = self
                        .formatter
                        .data
                        .get()
                        .end(self.formatter.style)
                        .parts(&next);
                    literal!(end_between)?;
                    value!(next)?;
                    literal!(end_after)
                } else {
                    // Pair(values[0], values[1]) = pair_before + values[0] + pair_between + values[1] + pair_after
                    let (before, between, after) = self
                        .formatter
                        .data
                        .get()
                        .pair(self.formatter.style)
                        .parts(&second);
                    literal!(before)?;
                    value!(first)?;
                    literal!(between)?;
                    value!(second)?;
                    literal!(after)
                }
            } else {
                value!(first)
            }
        } else {
            Ok(())
        }
    }

    fn write_len(&self) -> LengthHint {
        let mut count = 0;
        let item_length = self
            .values
            .clone()
            .map(|w| {
                count += 1;
                w.write_len()
            })
            .sum::<LengthHint>();
        item_length
            + self
                .formatter
                .data
                .get()
                .size_hint(self.formatter.style, count)
    }
}

#[cfg(all(test, feature = "provider_transform_internals"))]
mod tests {
    use super::*;
    use writeable::{assert_writeable_eq, assert_writeable_parts_eq};

    fn formatter(style: ListStyle) -> ListFormatter {
        ListFormatter {
            data: DataPayload::<ListFormatterPatternsV1Marker>::from_owned(
                crate::provider::test::test_patterns(),
            ),
            style,
        }
    }

    #[test]
    fn test_slices() {
        let formatter = formatter(ListStyle::Wide);
        let values = ["one", "two", "three", "four", "five"];

        assert_writeable_eq!(formatter.format(values[0..0].iter()), "");
        assert_writeable_eq!(formatter.format(values[0..1].iter()), "one");
        assert_writeable_eq!(formatter.format(values[0..2].iter()), "$one;two+");
        assert_writeable_eq!(formatter.format(values[0..3].iter()), "@one:two.three!");
        assert_writeable_eq!(
            formatter.format(values[0..4].iter()),
            "@one:two,three.four!"
        );

        assert_writeable_parts_eq!(
            formatter.format(values.iter()),
            "@one:two,three,four.five!",
            [
                (0, 1, parts::LITERAL),
                (1, 4, parts::ELEMENT),
                (4, 5, parts::LITERAL),
                (5, 8, parts::ELEMENT),
                (8, 9, parts::LITERAL),
                (9, 14, parts::ELEMENT),
                (14, 15, parts::LITERAL),
                (15, 19, parts::ELEMENT),
                (19, 20, parts::LITERAL),
                (20, 24, parts::ELEMENT),
                (24, 25, parts::LITERAL)
            ]
        );
    }

    #[test]
    fn test_into_iterator() {
        let formatter = formatter(ListStyle::Wide);

        let mut vecdeque = std::collections::vec_deque::VecDeque::<u8>::new();
        vecdeque.push_back(10);
        vecdeque.push_front(48);

        assert_writeable_parts_eq!(
            formatter.format(vecdeque.iter()),
            "$48;10+",
            [
                (0, 1, parts::LITERAL),
                (1, 3, parts::ELEMENT),
                (3, 4, parts::LITERAL),
                (4, 6, parts::ELEMENT),
                (6, 7, parts::LITERAL),
            ]
        );
    }

    #[test]
    fn test_iterator() {
        let formatter = formatter(ListStyle::Wide);

        assert_writeable_parts_eq!(
            formatter.format(core::iter::repeat(5).take(2)),
            "$5;5+",
            [
                (0, 1, parts::LITERAL),
                (1, 2, parts::ELEMENT),
                (2, 3, parts::LITERAL),
                (3, 4, parts::ELEMENT),
                (4, 5, parts::LITERAL),
            ]
        );
    }

    #[test]
    fn test_conditional() {
        let formatter = formatter(ListStyle::Narrow);

        assert_writeable_eq!(formatter.format(["Beta", "Alpha"].iter()), "Beta :o Alpha");
    }
}
