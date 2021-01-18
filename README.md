# cargo-semver

[![crates.io](https://img.shields.io/crates/v/cargo-semver)](https://crates.io/crates/cargo-semver)
[![codecov](https://codecov.io/gh/filipstefansson/cargo-semver/branch/master/graph/badge.svg?token=HSAldVxPvX)](https://codecov.io/gh/filipstefansson/cargo-semver)

**cargo-semver** is a cargo subcommand to help you read and update the version in your `Cargo.toml` file.

```console
$ cargo semver get
1.0.0

$ cargo semver bump patch
1.0.1
```

> **Important**: Running this CLI writes to `Cargo.toml`. Make sure to validate the version before commit.

## Installation

```console
$ cargo install cargo-semver --vers 1.0.0-alpha.3
```

## Usage

```console
# get the current version
$ cargo semver get

# bump the version with an optional pre-release
$ cargo semver bump [TYPE] [PRE-RELEASE]

# set a specific version
$ cargo set [VERSION]
```

### Update version

You can update the version in your `Cargo.toml` file using one of the subcommands:

```console
$ cargo semver bump major
2.0.0

$ cargo semver bump minor
2.1.0

$ cargo semver bump patch
2.1.1

$ cargo semver bump pre alpha
2.1.1-alpha.1
```

If you want to bump the version and add a pre-release version:

```console
$ cargo semver bump major alpha
2.0.0-alpha.1
```

### Updating the pre-release version

There are multiple ways of updating the pre-release version:

```console
$ cargo semver bump major alpha
2.0.0-alpha.1

$ cargo semver bump pre alpha
2.0.0-alpha.2

$ cargo semver bump pre
2.0.0-alpha.3

$ cargo semver bump pre beta
2.0.0-beta.1
```

### Set a specific version

If you want to set an exact version, use the `set` command:

```console
$ cargo semver set 2.1.3-beta.3
2.1.3-beta.3
```

## License

**cargo-semver** is provided under the MIT License. See LICENSE for details.
