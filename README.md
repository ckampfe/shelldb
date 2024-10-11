# shelldb

Record your `zsh` shell history in a SQLite database.
`shelldb` does this by hooking in to 2 `zsh` hooks, `preexec` and `precmd` as described [here](https://zsh.sourceforge.io/Doc/Release/Functions.html).

## motivation

This project is like [Atuin](https://github.com/atuinsh/atuin) but _only_ records shell history. 

I have no dislike of `Atuin`, but `Atuin` has a lot of features I don't need.

To that end, `shelldb` is:
1. _only_ shell history recording, nothing else
2. a minimal implementation I can understand 100% top-to-bottom
3. only available for `zsh`, because that's what I use

## installation

```sh
$ git clone https://github.com/ckampfe/shelldb.git
$ cd shelldb
$ cargo install --path . --force
$ echo "source /location/of/this_repo/shelldb.sh" >> ~/.zshrc
```

## options

Options are included in this README so you the end user can understand how `shelldb` works, but in reality you shouldn't have to use them, they are set in `shelldb.sh`.

The only reason to interact with the options as a user is to set a custom history database location, which you do by setting `SHELLDB_HISTORY_DB_PATH` to something like `/path/to/your/history.db`.

```sh
$ shelldb -h
record shell history in a SQLite database

Usage: shelldb [DATABASE_PATH] <COMMAND>

Commands:
  start   record the start of a shell command and the directory it was executed in
  finish  record a command's exit code
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [DATABASE_PATH]  set this to override the default history database path [env: SHELLDB_HISTORY_DB_PATH=]

Options:
  -h, --help  Print help
```

Start options:

```sh
at [ 22:59:21 ] ➜ shelldb start -h
record the start of a shell command and the directory it was executed in

Usage: shelldb start --command <COMMAND> --working-directory <WORKING_DIRECTORY>

Options:
      --command <COMMAND>                      the command to record
      --working-directory <WORKING_DIRECTORY>  the working directory of the command
  -h, --help                                   Print help
```

Finish options:

```sh
$ shelldb finish -h
record a command's exit code

Usage: shelldb finish --exit-code <EXIT_CODE> --id <ID>

Options:
      --exit-code <EXIT_CODE>  the exit code of a command that just completed
      --id <ID>                the ID of the since-completed command, so we can record its exit code
  -h, --help                   Print help
```
