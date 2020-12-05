if (( $+commands[fluvio] )); then
    __FLUVIO_COMPLETION_FILE="${ZSH_CACHE_DIR}/fluvio_completion"

    if [[ ! -f $__FLUVIO_COMPLETION_FILE ]]; then
        fluvio completions zsh >! $__FLUVIO_COMPLETION_FILE
    fi

    [[ -f $__FLUVIO_COMPLETION_FILE ]] && source $__FLUVIO_COMPLETION_FILE

    unset __FLUVIO_COMPLETION_FILE
fi

alias flv=fluvio
