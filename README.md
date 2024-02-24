# atoc2gtfs

![Build](https://github.com/heuristic-pedals/minigrep/actions/workflows/ci.yml/badge.svg?branch=main)
![Docs](https://github.com/heuristic-pedals/minigrep/actions/workflows/docs.yml/badge.svg?branch=main)
<a href="https://codecov.io/gh/heuristic-pedals/atoc2gtfs" > <img src="https://img.shields.io/codecov/c/github/heuristic-pedals/atoc2gtfs/main"/></a>

An ATOC.CIF to GTFS converter written in `rust`.

> Note: At this time there is no functional ATOC to GTFS conversion due to this repository being in active development.

## Introduction

Railway schedule data in Great Britain (GB) is supplied openly by the [Rail Delivery Group][RDG homepage] (RDG) in ATOC.CIF format (account sign-up is required). This data is shared by the RDG under a [Creative Commons England and Wales Public Licence][CC eng and wales]. The [RDG ATOC specifcation document][ATOC spec] is also openly available and defines the schedule's file and data structure.

Most modern transport network routing and analysis tools utilise data sources in [General Transit Feed Specification][GTFS docs] (GTFS) format. This introduces a significant blocker when working with GB rail schedule data.

Other open source ATOC to GTFS conversion tools exist written in a range of programming languanges including; [UK2GTFS] (written in R), [ATOCCIF2GTFS] (written in C#), and [atocciftogtfs] (written in python) - these are an inspiration for this work. This repository looks to build on such open tools using [`rust`][rust] to benefit from its performance and memory safe features.

`atoc2gtfs` is both a binary and library crate providing an ATOC.CIF to GTFS conversion capability. Usage instructions for the binary crate can be found below. The `atoc2gtfs` library API is described in more details within the documentation.

## Usage

> Note: This crate is not currently hosted on [cartes.io](https://crates.io). Therefore the following instructions require users to clone this repository and build the release binary (say using using `cargo`). All instructions are then relative to the directory's root.

An ATOC.CIF GB rail schedule can be converted to GTFS using the following command with 2 required arguments:

```bash
./target/release/atoc2gtfs <PATH TO INPUT ATOC> <PATH TO OUTPUS GTFS>
```

Where:

- `<PATH TO INPUT ATOC>` is the path to the input ATOC.CIF (.zip file).
- `<PATH TO OUTPUT GTFS>` is the desired output GTFS path (.zip file).

## License

All code in this repository is released under an [MIT licence][MIT]. All input data sources supplied by the RDG and any dervied GTFS outputs using this code are subject to the [Creative Commons England and Wales Public Licence][CC eng and wales]. This requires users to attribute the orignal data provider (RDG) and this repository.

[RDG homepage]: https://data.atoc.org/member-area
[CC eng and wales]: https://creativecommons.org/licenses/by/2.0/uk/legalcode.en-gb
[ATOC spec]: https://data.atoc.org/sites/all/themes/atoc/files/RSPS5046.pdf
[MIT]: https://github.com/heuristic-pedals/atoc2gtfs/blob/main/LICENSE
[GTFS docs]: https://gtfs.org/schedule/
[UK2GTFS]: https://itsleeds.github.io/UK2GTFS/
[ATOCCIF2GTFS]: https://github.com/odileeds/ATOCCIF2GTFS
[rust]: https://www.rust-lang.org
[atocciftogtfs]: https://pypi.org/project/atcociftogtfs/