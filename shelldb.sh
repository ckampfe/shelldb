#!/usr/bin/env zsh

# the following is ripped more or less wholesale from
# https://github.com/atuinsh/atuin/blob/main/crates/atuin/src/shell/atuin.zsh,
# which is licensed MIT
autoload -U add-zsh-hook

function _shelldb_preexec() {
  local id
  id=$(shelldb start --command "$1" --cwd "${PWD}")
  export SHELLDB_HISTORY_ID="$id"
}

function _shelldb_precmd() {
   local EXIT="$?"

    # if SHELL_HISTORY_ID is unset, return early
    [[ -z "${SHELLDB_HISTORY_ID:-}" ]] && return

    # - redirect stdout to /dev/null
    # - redirect stderr to stdout
    # - "finish" the command recording in the background,
    #   capturing the exit code and the command id
    (shelldb finish --exit-code "$EXIT" --id "$SHELLDB_HISTORY_ID" &) >/dev/null 2>&1

    export SHELLDB_HISTORY_ID=""
}

add-zsh-hook preexec _shelldb_preexec
add-zsh-hook precmd _shelldb_precmd
 