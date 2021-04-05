#!/usr/bin/env zsh

# Copy-pasted from https://gist.github.com/enisozgen/2109cc80ea9f405f80c6c383f2375e77


# Change last part of the command
# NOTE Creates sometime problem if there is same word in the input
ChangeLastCommand()
{
    [[ -z  "${2// }" ]] && printf "%s" "$(echo "${1}${3}")" || printf "%s" "$(echo "${1/$2/ $3}")"
}

SendLastCommandAfterPIPE()
{
    # Send last command after pipe
    INPUT_STRING="${1}"
    a=("${(@s/|/)${INPUT_STRING}}") # | modifier
    printf "%s" "$(echo "${a[-1]}")"
}


# Is there pipe in the input string
IsPipeExist()
{
    INPUT_STRING=$1
    if [[ $INPUT_STRING == *\|* ]]; then
        NAVI_PIPE="true"
    else
        NAVI_PIPE="false"
    fi
}


IsOnlySpace ()
{
    if [[ -z  "${1// }" ]] ; then
        return 0
    else
        return 1
    fi
}

NaviUISearch()
{
    NAVI_RET=$(printf "%s" "$(navi --print --fzf-overrides '--no-select-1' --query "${1}" </dev/tty)")
    printf ${NAVI_RET}
}


NaviOutputControl ()
{
    if ! [[ -z "$1" ]] ; then
        return 0
    else
        return 1
    fi
}



SmartNavi()
{
    if  IsOnlySpace $1 ; then
        NAVI_RET=$(printf "%s" "$(navi --print  --fzf-overrides '--no-select-1' </dev/tty)")
    else
        NAVI_RET=$(printf "%s" "$(navi --print --best-match --fzf-overrides '--no-select-1' --query "${1}" </dev/tty)")
    fi


    # Return warning if there is no output else return best match
    if NaviOutputControl ${NAVI_RET}; then
        printf ${NAVI_RET}
    else
        printf "Navi Returned Empty"
    fi
}


_call_navi() {
    local selected
    if [ -n "$LBUFFER" ]; then
        if selected="$(printf "%s" "$(navi --print --fzf-overrides '--no-select-1' --query "${LBUFFER}" </dev/tty)")"; then
            LBUFFER="$selected"
        fi
    else
        # If there is not any word on list
        if selected="$(printf "%s" "$(navi --print </dev/tty)")"; then
            LBUFFER="$selected"
        fi
    fi
    region_highlight=("P0 100 bold")
    zle redisplay
}

_call_smart_navi() {

    # set -x
    INPUT_STRING=$LBUFFER
    IsPipeExist ${INPUT_STRING}


    # Is there some written stuff in LBUFFER ?
    if ! [ -z "$INPUT_STRING" ] ; then
        # If last navi output same as current input
        # Use this part when you don't like navi best match
        if [ "${LASTWIDGET}"  = "_call_smart_navi" ]  && [ "${OUTPUT_STRING}"  = "$INPUT_STRING" ];then
            LBUFFER_LAST_COMMAND=$(SendLastCommandAfterPIPE "${INPUT_STRING}")

            # Searching with same input as before but this time we are using navi interactive UI since navi didn't return us what we want
            OUTPUT_STRING=$(NaviUISearch  ${PREVIOUS_LAST})
            OUTPUT_STRING=$(ChangeLastCommand "$INPUT_STRING" "$LBUFFER_LAST_COMMAND" "$OUTPUT_STRING")

        else
            # First search always start from here!!!
            if [ "${NAVI_PIPE}" = "false" ] ; then

                # LBUFFER_LAST_COMMAND=$(SendLastCommandAfterPIPE "${INPUT_STRING}")
                # PREVIOUS_LAST=$LBUFFER_LAST_COMMAND

                # Remember what was last command after pipe
                PREVIOUS_LAST=$INPUT_STRING
                OUTPUT_STRING=$(SmartNavi ${INPUT_STRING})

            else
                LBUFFER_LAST_COMMAND=$(SendLastCommandAfterPIPE "${INPUT_STRING}")

                # Remember what was last command after pipe
                PREVIOUS_LAST=$LBUFFER_LAST_COMMAND

                OUTPUT_STRING=$(SmartNavi ${LBUFFER_LAST_COMMAND})
                OUTPUT_STRING=$(ChangeLastCommand "$INPUT_STRING" "$LBUFFER_LAST_COMMAND" "$OUTPUT_STRING")

            fi
        fi
        LBUFFER="$OUTPUT_STRING"
    else
        # There is nothing use default navi command
        _call_navi
    fi

    region_highlight=("P0 100 bold")
    zle redisplay
}


zle -N _call_smart_navi
bindkey '^g' _call_smart_navi