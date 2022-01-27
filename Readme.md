# Watchout

A cross platform reloading image viewer combined with a command executor.

<img src="resources/icon.png" width="256" height="256" />

Watchout will do any of the following:

- Run a `command` when any file in a set of `Paths` changes. After the command is done running, it will reload and display an image. Imagine you're working on a SQL query to generate a chart. Whenever you save the SQL file *Watchout* would run a specific command (say `generate-chart.py query.sql`) and upon completion would reload
the image that was generated by `generate-chart.py`.
- Check if an image on disk changed and if that is the case re-display the image.

Watchout should run cross platform, but it has only been tested on macOS.

## Demo

https://user-images.githubusercontent.com/132234/151373030-d46e33bd-b0f5-46a7-a8ab-73359ef74309.mov

(In this demo you can see watchout running `cargo run --example image` whenever the `image.rs` file is saved. The `image` example writes `output.png` to disk which is displayed by watchout.

## Usage

Watchout has to be started from the terminal, quick example:

``` sh
watchout image -img /path/to/image.png
```

This will start watchout with an image and redisplay the image if it changes.

Running with a command. Below will run `cargo run --example generate_image`  whenever anything in `/proj/bam/src` or `/proj/bam/examples` changes. Once it finished running, it will reload `/proj/bam/output.png`.

``` sh
watchout command-image -c "cargo run --example generate_image" -w /proj/bam/src -w /proj/bam/examples -i /proj/bam/output.png
```

There's another mode where the output of the `cmd` can define the image to be displayed. In this case, the last line of output from `cmd` should only be the path to the image that is to be displayed:

``` sh
watchout command-output -c "cargo run --example generate_image" -w /proj/bam/src
```

In this example, whenever `/proj/bam/src` changes, *watchout* will run `cargo run --example generate_image` and then take the last line of output from running `cargo run --example generate_image`, interpret that as the path to an image, and load that image.

## Full Usage

``` sh
USAGE:
    watchout [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    command-image     Perform [cmd] when the [watch] folder (recursively) changes and then
                      reload [img]
    command-output    Perform [cmd] when the [watch] folder (recursively) changes and then
                      reload the image at the path in the last line of the [cmd] output
    help              Print this message or the help of the given subcommand(s)
    image             Reload [img] when it changes
```

## Building

``` sh
cargo build --release
```

If you want your executable to have a proper icon, you can use `cargo bundle` (`cargo install cargo-bundle`):

``` sh
cargo bundle --release
```

Or, on macOS

``` sh
./build_mac.sh
```

## Todo

- [ ] Better error handling (e.g. displaying an error)
- [ ] Allow manually reloading an image / running the command
- [ ] Allow zooming / panning of the image
- [ ] Show the log output in the app

## Libraries

Watchout was build using these fine libraries

- [druid](https://github.com/linebender/druid)
- [notify](https://github.com/notify-rs/notify)
- [clap](https://crates.io/crates/clap)
- [anyhow](https://crates.io/crates/anyhow)
- [crossbeam](https://crates.io/crates/crossbeam)
- [tracing](https://crates.io/crates/tracing)
