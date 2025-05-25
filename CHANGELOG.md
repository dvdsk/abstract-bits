# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added


### Changed


### Removed


## [0.2.0] - 2025-05-25

### Changed

- Fields that control presence of an `Option` or length of a `Vec` no longer
  need to be named reserved.
- *Breaking* Instead of `controls` fields determining `Option` presence are now
  annotated with `presence_of`.
