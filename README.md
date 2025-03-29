# sdiff

Find the differences between two sequences.

A diffing function that finds the longest common subsequence (LCS) of two sequences. The found LCS
can be easily transformed to a shortest edit script (SES). The implementation is based on the
[difference algorithm by Eugene W. Myers].

This crate is developed for being used in the [`asserting`] crate as none of the many existing
similar crates fulfills all requirements.

The requirements are:

* no-std: no dependency to the std-lib
* dual licensed under MIT or Apache-2.0
* simple to use
* fast to compile
* small binary size
* minimal memory usage

<!-- External Links -->

[`asserting`]: https://github.com/innoave/asserting

[difference algorithm by Eugene W. Myers]: http://www.xmailserver.org/diff2.pdf

[James Coglan Blog Post]: https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/
