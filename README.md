# rugrep

> This is a pet-project of mine that I used to hone my Rust-jutsu and become a proud rustian. Don't rely on it for production usage!

Grep-like program based on the sample from [The Rust Programming Language book](https://doc.rust-lang.org/book/).

## Installing

You can install without the toolchain by downloading the latest release and extracting it to a binary PATH, e.g.:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://github.com/wwmoraes/rugrep/releases/<tag>/rugrep.tar.gz | tar -xf - -C <desired-bin-path>
```

IF you happen to have a [working toolchain](#Developing) you can install from crates.io or build.

> TODO: push it into crates.io so, the below installation applies as well
>> Installing from crates.io with cargo:
>>
>> ```bash
>> cargo install rugrep
>> rugrep <text> <file1> [file2...fileN]
>> ```

Building and installing from source:

```bash
git clone git@github.com:wwmoraes/rugrep.git
cd rugrep/
cargo install --path .
```

Both will install per default at `~/.cargo/bin`.

### Getting started

Simply run with the text you want to search, and one or more files to match against:

```bash
rugrep <text> <file1> [file2...fileN]
```

The output will be the matching file name(s), line number(s) and line content(s).

## Developing

You'll need a working rust toolchain instaled. If you don't, use rustup.rs, the official toolchain installer:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

With a working toolchain, just:

```bash
git clone https://github.com/wwmoraes/rugrep.git
cd rugrep/
```

Then open up your favorite editor and hack away!

You can run the code directly without building with `cargo run [args]`. If you happen to use double-dash flags, use the form `cargo run -- [args]`.

### Building

Use cargo for building:

```bash
cargo build
```

Or to build a optimized release version:

```bash
cargo build --release
```

You should get a success output and a working binary at `target/[debug|release]/rugrep`.

If you want to do a clean build, use `cargo clean` before.

### Testing

To fully test your code, use `cargo test`. You can also specify which test to run with `cargo test [filter]`.

### Documentation

You can generate the documentation with `cargo doc`, which will be available in `target/doc/rugrep`, or update and open it in a single command:

```bash
cargo doc --open
```

### Profiling

The `Makefile` includes targets for profiling with `heaptrack` and Valgrind's `massif` tool, both memory profilers with a lot of bells and whistles like allocations, leaks and flame graph.

```bash
# Heaptrack targets
make ht # runs ht-build and ht-view
make ht-build # generate new heaptrack profiling data
make ht-view # analyzes previously generated heaptrack profiling data
make ht-compare # compares the latest two heaptrack profiling data
make ht-clean # clean up all heaptrack's profiling data

# Massif targets
make massif # runs massif-build and massif-view
make massif-build # generate new massif profiling data
make massif-view # analyzes previously generated massif profiling data
make massif-clean # clean up all massif's profiling data

# General profiling targets
make profile-clean # Cleans both heaptrack and massif profiling data
```

All these targets will work on the debug target, unless you provide `TARGET=release`.

### Deploying / Publishing

To publish a new release to crates.io:

```bash
cargo clean
cargo package
cargo publish
```

> You'll need to be logged in by getting an crates.io API token and logging in with `cargo login <your-api-token>`.

These commands will pack the release and push it to crates.io.

## Features

`rugrep` is a `grep` wannabe:

* searches text case-sensitively on given files
* can search case-insensitively with the `-i` flag
* outputs verbose info with the `-v` flag
* TODO: read from stdin (so it can be piped)
* TODO: move file and line info to the verbose flag, outputting only the matched lines per default
* TODO: regular expressions 🖤️

## Contributing

If you'd like to contribute, please fork the repository and use a feature branch.

## Links

* Project homepage: [wwmoraes.github.com/rugrep](https://wwmoraes.github.com/rugrep/)
* Repository: [github.com/wwmoraes/rugrep](https://github.com/wwmoraes/rugrep/)
* Issue tracker: [github.com/wwmoraes/rugrep/issues](https://github.com/wwmoraes/rugrep/issues)
  * In case of sensitive bugs like security vulnerabilities, please
    [contact me](http://scr.im/wwmoraes) directly instead of using issue tracker. We value your effort
    to improve the security and privacy of this project!
* Inspiration:
  * [Rust book project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
  * [crates.io minigrep](http://crates.io/crates/minigrep)

## Licensing

The code in this project is licensed under MIT license.
