# simavr-sys

[![Crates.io](https://img.shields.io/crates/v/simavr-sys.svg)](https://crates.io/crates/simavr-sys)
[![Build Status](https://travis-ci.org/dylanmckay/simavr-sys.svg?branch=master)](https://travis-ci.org/dylanmckay/simavr-sys)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Bindings to the [simavr](https://github.com/buserror/simavr) AVR simulator.

[Documentation](https://docs.rs/simavr-sys/)

## Versioning scheme

The crate published to crates.io will always include the major and minor simavr version in
the crates.io version.

The revision is specific to this crate.

Versions look like this

`<simavr-major>.<simavr-minor>.<simavr-sys-revision>`

So, if we've made two revisions with simavr `1.5.9`, then the new version
should be `1.5.1`.

When a new minor version is released, the minor version should tick back to zero.

