use super::*;
use asserting::prelude::*;

#[cfg(feature = "std")]
mod properties {
    use super::*;
    use crate::std::string::String;
    use proptest::prelude::*;

    #[test]
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn max_sequence_length_value() {
        assert_that!(max_sequence_length())
            .is_equal_to(2 * ((isize::MAX as f64).sqrt() as usize - 2));
    }

    proptest! {
        #[test]
        fn diffing_any_two_strings_does_not_panic(
            left in any::<String>(),
            right in any::<String>(),
        ) {
            _ = diff_str(&left, &right);
        }

        #[test]
        fn the_length_of_a_trace_is_less_than_or_equal_two_the_sum_of_the_length_of_the_two_sequences(
            left in prop::collection::vec(any::<i32>(), 0..=600),
            right in prop::collection::vec(any::<i32>(), 0..=600),
        ) {
            let trace = find_shortest_trace(&left, &right);

            prop_assert!(trace.len() <= left.len() + right.len(),
                "length of trace is at most the sum of the length of the sequences: {:?} <= {:?} + {:?}",
                trace.len(), left.len(), right.len()
            );
        }
    }
}

mod diff_strings {
    use super::*;

    #[test]
    fn both_empty() {
        let left = "";
        let right = "";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([Diff::Both {
            left_index: 0,
            right_index: 0,
            length: 0,
        }]);
    }

    #[test]
    fn equal() {
        let left = "tation facilisi commodo reprehenderit";
        let right = "tation facilisi commodo reprehenderit";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([Diff::Both {
            left_index: 0,
            right_index: 0,
            length: 37,
        }]);
    }

    #[test]
    fn nothing_in_common() {
        let left = "ABCDEFG";
        let right = "MNOPQ";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Left {
                index: 0,
                length: 7,
            },
            Diff::Right {
                index: 0,
                length: 5,
            },
        ]);
    }

    #[test]
    fn swapped_chars() {
        let left = "ABCD";
        let right = "ABDC";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Both {
                left_index: 0,
                right_index: 0,
                length: 2,
            },
            Diff::Left {
                index: 2,
                length: 1,
            },
            Diff::Both {
                left_index: 3,
                right_index: 2,
                length: 1,
            },
            Diff::Right {
                index: 3,
                length: 1,
            },
        ]);
    }

    #[test]
    fn replaced_char() {
        let left = "ABCE";
        let right = "ABDE";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Both {
                left_index: 0,
                right_index: 0,
                length: 2,
            },
            Diff::Left {
                index: 2,
                length: 1,
            },
            Diff::Right {
                index: 2,
                length: 1,
            },
            Diff::Both {
                left_index: 3,
                right_index: 3,
                length: 1,
            },
        ]);
    }

    #[test]
    fn removed_chars() {
        let left = "ABCDEFG";
        let right = "ABFG";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Both {
                left_index: 0,
                right_index: 0,
                length: 2,
            },
            Diff::Left {
                index: 2,
                length: 3,
            },
            Diff::Both {
                left_index: 5,
                right_index: 2,
                length: 2,
            },
        ]);
    }

    #[test]
    fn inserted_chars() {
        let left = "ABCEFG";
        let right = "ABCXYZEFG";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Both {
                left_index: 0,
                right_index: 0,
                length: 3,
            },
            Diff::Right {
                index: 3,
                length: 3,
            },
            Diff::Both {
                left_index: 3,
                right_index: 6,
                length: 3,
            },
        ]);
    }

    #[test]
    fn all_inserted() {
        let left = "";
        let right = "ABCDEFG";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([Diff::Right {
            index: 0,
            length: 7,
        }]);
    }

    #[test]
    fn all_deleted() {
        let left = "ABCDEFGH";
        let right = "";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([Diff::Left {
            index: 0,
            length: 8,
        }]);
    }

    #[test]
    fn moved_block_of_chars_to_end() {
        let left = "ABCDEFG";
        let right = "AEFGBCD";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Both {
                left_index: 0,
                right_index: 0,
                length: 1,
            },
            Diff::Left {
                index: 1,
                length: 3,
            },
            Diff::Both {
                left_index: 4,
                right_index: 1,
                length: 3,
            },
            Diff::Right {
                index: 4,
                length: 3,
            },
        ]);
    }

    #[test]
    fn moved_block_of_chars_to_start() {
        let left = "ABCDEFG";
        let right = "CDEABFG";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Left {
                index: 0,
                length: 2,
            },
            Diff::Both {
                left_index: 2,
                right_index: 0,
                length: 3,
            },
            Diff::Right {
                index: 3,
                length: 2,
            },
            Diff::Both {
                left_index: 5,
                right_index: 5,
                length: 2,
            },
        ]);
    }

    #[test]
    fn abcabba_and_cbabac() {
        let left = "abcabba";
        let right = "cbabac";

        let diffs = diff_str(left, right);

        assert_that!(diffs).contains_exactly([
            Diff::Left {
                index: 0,
                length: 2,
            },
            Diff::Both {
                left_index: 2,
                right_index: 0,
                length: 1,
            },
            Diff::Right {
                index: 1,
                length: 1,
            },
            Diff::Both {
                left_index: 3,
                right_index: 2,
                length: 2,
            },
            Diff::Left {
                index: 5,
                length: 1,
            },
            Diff::Both {
                left_index: 6,
                right_index: 4,
                length: 1,
            },
            Diff::Right {
                index: 5,
                length: 1,
            },
        ]);
    }
}
