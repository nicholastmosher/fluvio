if (( $+commands[fluvio] )); then
    __FLUVIO_COMPLETION_FILE="${ZSH_CACHE_DIR}/fluvio_completion"

    if [[ ! -f $__FLUVIO_COMPLETION_FILE ]]; then
        fluvio completions zsh >! $__FLUVIO_COMPLETION_FILE
    fi

    if [[ -f $__FLUVIO_COMPLETION_FILE ]]; then
        autooad -U compinit
        compinit
        source <(cat $__FLUVIO_COMPLETION_FILE | sed '$d')
        compdef _fluvio fluvio
    fi

    unset __FLUVIO_COMPLETION_FILE
fi

alias flv=fluvio
