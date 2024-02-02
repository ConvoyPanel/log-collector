# Changelog

This project follows [Semantic Versioning](http://semver.org) guidelines.

## v2.0.1

### Changes

- Added brand message to entrypoint.
- Removed dead code.
- Fixed panics when a log file doesn't exist.

## v2.0.0

### Changes

- Decreased log size count from 20 MiB to 5 MiB
- Added log collection of system information
- Lots of fucking refactors. I spent a whole damn afternoon/evening on this shit.

## v1.1.0

### Added

- Cross-platform (distribution) support
    - Specifically, this release removed the dependence on OpenSSL, which some distributions may have a drastically
      older version (v1) compared to the required v3 version of OpenSSL.

## v1.0.0

Initial release