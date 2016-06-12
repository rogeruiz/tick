# Tick CLI for tracking your time

[![Project Build Status](https://travis-ci.org/rogeruiz/tick.svg?branch=master)](https://travis-ci.org/rogeruiz/tick)

I use `tick` to track all my time using `tmux` sessions. But `tick` can be used
without `tmux`, `node` or any other dependency. The `tick` CLI is written in Rust.

## Installation

To install `tick` on your system, you will need to compile it from source _for
now_.

```shell
# Clone the repository
git clone https://github.com/rogeruiz/tick.git

# Build the release
cd tick
cargo build --release

# Install and make tick executable
cp ./target/release/tick /usr/local/bin/tick
chmod +x /usr/local/bin/tick
```

## Motivation

I track my time a lot while using the terminal using a wrapper around `tmux`.
The wrapper I have is [a shell script called `tux`][tux-src]. While the wrapper
works great, it depends on `clocker` and `node` to handle time tracking.

The main motivation around writing this was to remove the `node` and `clocker`
dependencies from `tux` along with adding customizable exporting mechanisms.
Tracking your time can be hard enough, so `tick` tries making it a lot easier.

[tux-src]: https://github.com/rogeruiz/.files/blob/master/bin/tux "`.files/bin/tux` Source"

## Inspiration

This project would not be possible without being inspired by other's work.

- `clocker` - [repository][clocker-repo]
- `watson` - [repository][watson-repo]

[clocker-repo]: https://github.com/substack/clocker "Clocker Repository"
[watson-repo]: https://github.com/TailorDev/Watson "Watson Repository"

