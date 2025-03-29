# sdiff

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
![MSRV][msrv-badge]
[![code coverage][code-coverage-badge]][code-coverage-url]

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

<!-- Badges and related URLs -->

[crates-badge]: https://img.shields.io/crates/v/sdiff.svg

[crates-url]: https://crates.io/crates/sdiff

[docs-badge]: https://docs.rs/sdiff/badge.svg

[docs-url]: https://docs.rs/sdiff

[msrv-badge]: https://img.shields.io/crates/msrv/sdiff?color=chocolate

[code-coverage-badge]: https://codecov.io/github/innoave/sdiff/graph/badge.svg?token=o0w7R7J0Op

[code-coverage-url]: https://codecov.io/github/innoave/sdiff

<!-- External Links -->

[`asserting`]: https://github.com/innoave/asserting

[difference algorithm by Eugene W. Myers]: http://www.xmailserver.org/diff2.pdf

[James Coglan Blog Post]: https://blog.jcoglan.com/2017/02/12/the-myers-diff-algorithm-part-1/
