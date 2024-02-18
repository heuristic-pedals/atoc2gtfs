# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Public `config::new()` API (improved config build functionality).
- Initial MCA file parsing.
- Initial MSN file parsing.

## [0.2.1] - 2024-02-18

### Changed
- Refactored `check_zip()` to `check_extension()` - resolved [#8].
- Fixed codecov shield link in README - resolved [#3].

## [0.2.0] - 2024-02-17

### Added
- Initial CLI interface with defences.
- `config` build API + documentation.

### Changed
- Readme contains intial introduction and usage sections.
- Readme clarifies license considerations.

## [0.1.0] - 2024-01-31

### Added

- Initialised cargo project.
- CI set-up.
- Documentation and GitHub pages set-up.
- Codecov set-up.
- Added issue and PR templates.

[unreleased]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/heuristic-pedals/atoc2gtfs/releases/tag/v0.1.0

[#3]: https://github.com/heuristic-pedals/atoc2gtfs/issues/3
[#8]: https://github.com/heuristic-pedals/atoc2gtfs/issues/8