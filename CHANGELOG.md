# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Initial MCA file parsing.
- Initial MSN file parsing.

## [0.5.0] - 2024-03-02

### Added
- CLI help details using `-h` or `--help` - closes [#13].

## [0.4.0] - 2024-02-28

### Added
- Required ATOC file checks during `Config` initialisation - closes [#11].
- `zip` added as a dependency.
- Initial `run` function - to call complete conversion pipeline
- Itegration test covering `Config` and `run`.

### Changed
- Updated `Config` docs with ATOC file checks.

### Fixed
- Typo in README.md

## [0.3.0] - 2024-02-24

### Added
- Public `Config::new()` API (improved config build functionality). - resolved [#7]

### Changed
- `Config::build()` refactored to `Config::build_from_cli()` to be more explicit.
- `cli` module refactored to `setup` to group both `build_from_cli()` and `new()` functions.

### Fixed
- Typo in README.md

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

[unreleased]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/heuristic-pedals/atoc2gtfs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/heuristic-pedals/atoc2gtfs/releases/tag/v0.1.0

[#3]: https://github.com/heuristic-pedals/atoc2gtfs/issues/3
[#8]: https://github.com/heuristic-pedals/atoc2gtfs/issues/8
[#7]: https://github.com/heuristic-pedals/atoc2gtfs/issues/7
[#11]: https://github.com/heuristic-pedals/atoc2gtfs/issues/11
[#13]: https://github.com/heuristic-pedals/atoc2gtfs/issues/13