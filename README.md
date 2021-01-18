# cargo-semver

[![crates.io](https://img.shields.io/crates/v/cargo-semver)](https://crates.io/crates/cargo-semver)
[![codecov](https://codecov.io/gh/filipstefansson/cargo-semver/branch/master/graph/badge.svg?token=HSAldVxPvX)](https://codecov.io/gh/filipstefansson/cargo-semver)

**cargo-semver** is a cargo subcommand to help you update the version in your `Cargo.toml` file.

```console
$ cargo semver
1.0.0

$ cargo semver patch
1.0.1
```

> **Important**: Running this CLI writes to `Cargo.toml`. Make sure to validate the version before commit.

## Installation

```console
$ cargo install cargo-semver
```

## Usage

```console
$ cargo semver # gets the current version
$ cargo semver major|minor|patch|pre # bumps the version
$ cargo set [VERSION] # sets a specific version
```

### Update version

You can update the version in your `Cargo.toml` file using one of the subcommands:

```console
$ cargo semver major
2.0.0

$ cargo semver minor
2.1.0

$ cargo semver patch
2.1.1

$ cargo semver pre alpha
2.1.1-alpha.1
```

If you want to bump the version and add a pre-release version in one command you can use the `pre` flag:

```console
$ cargo semver major --pre alpha
2.0.0-alpha.1
```

### Updating the pre-release version

There are multiple ways of updating the pre-release version:

```console
$ cargo semver major --pre alpha
2.0.0-alpha.1

$ cargo semver pre alpha
2.0.0-alpha.2

$ cargo semver pre
2.0.0-alpha.3

$ cargo semver pre beta
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
