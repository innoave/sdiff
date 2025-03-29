use super::*;
use asserting::prelude::*;

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
