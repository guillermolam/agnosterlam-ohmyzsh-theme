### Weather information
get_weather_info() {
  local weather_info
  weather_info=$(weather_info)
  echo "$weather_info"
}

### Prompt components
# Begin a segment
# Takes two arguments, background and foreground. Both can be omitted,
# rendering default background/foreground.
prompt_segment() {
  local bg fg
  [[ -n $1 ]] && bg="%K{$1}" || bg="%k"
  [[ -n $2 ]] && fg="%F{$2}" || fg="%f"
  if [[ $CURRENT_BG != 'NONE' && $1 != $CURRENT_BG ]]; then
    print -n " %{$bg%F{$CURRENT_BG}%}$SEGMENT_SEPARATOR%{$fg%} "
  else
    print -n "%{$bg%}%{$fg%} "
  fi
  CURRENT_BG=$1
  [[ -n $3 ]] && print -n $3
}

# End the prompt, closing any open segments
prompt_end() {
  if [[ -n $CURRENT_BG ]]; then
    print -n " %{%k%F{$CURRENT_BG}%}$SEGMENT_SEPARATOR"
  else
    print -n "%{%k%}"
  fi
  print -n "%{%f%}"
  CURRENT_BG=''
}

# Main prompt
build_prompt() {
  local weather_info icon city temp
  weather_info=$(get_weather_info)
  IFS=' ' read -r icon city temp <<< "$weather_info"
  print -n "\n"
  prompt_segment blue white "$icon"
  prompt_segment white blue "$city"
  prompt_segment blue white "${temp}°C"
  prompt_end
}

PROMPT='%{%f%b%k%}$(build_prompt) '
