#!/bin/bash
# Bash completion for git-sdif command
# This provides branch name completion for git-sdif parameters

# Function to complete git-sdif command (both standalone and git subcommand)
_git_sdif()
{
    local cur prev words cword
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    cword=$COMP_CWORD

    # For git subcommand, adjust cword to account for 'git' being words[0]
    if [[ "${COMP_WORDS[0]}" == "git" ]]; then
        ((cword--))
    fi

    # Only complete for first two parameters (after the command itself)
    if [[ $cword -gt 2 ]]; then
        return 0
    fi

    # Complete with git references for both first and second parameters
    if [[ $cword -eq 1 || $cword -eq 2 ]]; then
        # Get all branches and tags
        local refs=$(git for-each-ref --format='%(refname:short)' refs/heads/ refs/remotes/ refs/tags/ 2>/dev/null)
        COMPREPLY=($(compgen -W "$refs" -- "$cur"))
        return 0
    fi
}

# Register completion for git-sdif standalone command
complete -o nospace -F _git_sdif git-sdif

# Register with git's completion system for 'git sdif' subcommand
if declare -f __git_complete >/dev/null 2>&1; then
    __git_complete sdif _git_sdif
fi

# Also register for when called as 'sdif' if it exists
if command -v sdif >/dev/null 2>&1; then
    complete -o nospace -F _git_sdif sdif
fi
