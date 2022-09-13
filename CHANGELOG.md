# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- [CI] Github action for ubuntu/windows/macos + stable/1.56

### Changed
- minimum Rustc version is now 1.56
- rust edition is now "2018"

### Removed
- [CI] travis build is removed

## v0.4.7

- Fix test bug #4
- Fix test bug #3

## v0.4.6

- Add test for `.median()` and `.quantile()`. 
- Add index range check.

## v0.4.5

- Fix dependent crate.

## v0.4.4

- Add `.median()` and `.quantile()`. They are quite fast, only take 3-5 us on 16-bit values.
- Add `.top_k_ranges()` which is faster than `.top_k()` in worst case.
- Add `.sum_experiment1()`, `.mean_experiment1()` and `.variance_experiment1()`.
- Add `.sum_experiment2()`.
- Add `.sum_experiment3()`.
- Add `BENCH.md` bench report.
- Add `BENCH_SUM.md` bench report.

## v0.4.3

- Add `.bit_len()`.
- Add `.dim()`.
- Move examples to crate's document top.
- Add test for `.top_k()`, `.max_k()` and `min_k()`.
- Suppress warnings.

## v0.4.2

- Add `.top_k()`, `.max_k()` and `min_k()`.

## v0.4.1

- Add `.search()` and `.search_prefix()`.

## v0.4.0

- Add `.count()`, `.count_prefix()`, `.count_lt()`, `.count_gt()` and `.count_range()`.
- [INCOMPATIBLE] WaveletMatrix::new() takes `&Vec<u64>`, instead of `Vec<u64>`

## v0.3.0

- [INCOMPATIBLE] .select() now returns .len() instead of None.
