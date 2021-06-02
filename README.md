# Tick CLI for tracking your time

![Angry clock eats person](angry-clock-eats-person.png)

I use `tick` to track all my time using `tmux` sessions. But `tick` can be used
without `tmux`, `node` or any other dependency. The `tick` CLI is written in Rust.

Info | Description
--- | ---
[![Project Build Status](https://travis-ci.org/rogeruiz/tick.svg?branch=master)](https://travis-ci.org/rogeruiz/tick) | Project build status for Tick CLI
[Installation](#installation) | Installing Tick CLI.
[Motivation](#motivation) | Why use Tick CLI?
[Commands](#commands) | Using Tick CLI.
[Inspiration](#inspiration) | Everything is a remix, including Tick CLI.
[Contributing](CONTRIBUTING.md) | Contribute to Tick CLI.
[License](LICENSE.md) | License for Tick CLI.

## Installation

To install Tick, you can either compile it from source or download the binary
from [the releases page][tick-releases] for the release you want and the
platform you need.

[tick-releases]: https://github.com/rogeruiz/tick/releases "The releases for this repo"

### Compiling Tick from source

The steps are pretty straight-forward as long as you are within the realm of
[Tier 1 support][rustlang-tier1] for the Rust compiler.

[rustlang-tier1]: https://forge.rust-lang.org/platform-support.html#tier-1 "Rust Platform Support"

```shell
# Clone the repository.
>_ git clone https://github.com/rogeruiz/tick.git

>_ cd tick

# Setup the database, environment, and run any migrations.
>_ cargo install diesel_cli
>_ cp ./.env.example ./.env
>_ diesel setup --database-url "${TICK_DATABASE_FILE}"

# Build the release.
>_ cargo build --release

# Install in your path.
>_ cp ./target/release/tick /usr/local/bin/tick
```

### Troubleshooting the first run of tick

Currently when you run Tick for the first time and haven't setup the database
tables for the timers nor exported the path to your database via
`$TICK_DATABASE_FILE` in your shell, you're going to run into a Rust panic.
Remember to setup your environment with the right variable set to the path to
your SQLite database and make sure you've run the migration found in this
repository. Use the `diesel_cli` cargo package to setup the database from within
the cloned project.

```sh
>_ cargo install diesel_cli
>_ cp ./.env.example ./.env
>_ diesel setup --database-url "${TICK_DATABASE_FILE}"
```

## Motivation

I track my time a lot while using the terminal using a wrapper around `tmux`.
The wrapper I have is [a shell script called `tux`][tux-src]. While the wrapper
works great, it depends on `clocker` and `node` to handle time tracking.

The main motivation around writing this was to remove the `node` and `clocker`
dependencies from `tux` along with adding customizable exporting mechanisms.
Tracking your time can be hard enough, so Tick tries making it a lot easier.

[tux-src]: https://github.com/rogeruiz/.files/blob/master/bin/tux "`.files/bin/tux` Source"

## Commands

Run `tick --help` to see all the available commands you can use. Below is an
example workflow of how you would use Tick.

```sh
>_ tick [ -v ] start --name my-timer [ --message "I can do the thing!" ]
>_ tick [ -v ] status
>_ tick [ -v ] stop --name my-timer [ --message "I did the thing!" ]
>_ tick [ -v ] stop [ --message "I did the thing!" ] # without a name argument stops the latest running timer
>_ tick [ -v ] list
>_ tick [ -v ] remove --id $( tick list | tail -1 | awk '{ print $1 }' ) # delete the latest timer by Timer ID
```

## Inspiration

This project would not be possible without being inspired by other's work.

- `clocker` - [repository][clocker-repo]
- `watson` - [repository][watson-repo]

[clocker-repo]: https://github.com/substack/clocker "Clocker Repository"
[watson-repo]: https://github.com/TailorDev/Watson "Watson Repository"

