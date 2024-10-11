# shelldb

Record your `zsh` command history to a SQLite database.

## installation

First, [install Rust](https://www.rust-lang.org/tools/install).

Then:

```sh
$ git clone https://github.com/ckampfe/shelldb.git
$ cd shelldb
$ cargo install --path . --force
$ echo "source /location/of/this_repo/shelldb.sh" >> ~/.zshrc
```

## options

The environment variable `SHELLDB_HISTORY_DB_PATH` allows you to override the default and set a custom history database location. You should set it in your `.zshrc` before sourcing `shelldb.sh`, and it should look like: `/path/to/your/history.db`.

Run `shelldb -h` to see the other options if you're curious about how `shelldb` works.

## how

`shelldb` hooks in to 2 `zsh` hooks, `preexec` and `precmd` as described [here](https://zsh.sourceforge.io/Doc/Release/Functions.html). The first phase, during `preexec`, records the command, the working directory the command was executed in, and the start time. This first phase returns an id unique to that command that the subsequent `precmd` phase references to then set the exit code and the finish time when the command finishes executing.

## motivation

This project is like [Atuin](https://github.com/atuinsh/atuin) but _only_ records shell history. 

I have no dislike of `Atuin`, but `Atuin` has a lot of features I don't need.

To that end, `shelldb` is:
1. _only_ shell history recording, nothing else
2. a minimal implementation I can 100% understand top-to-bottom
3. only available for `zsh`, because that's what I use

## is it any good?

yes.
