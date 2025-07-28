# ~/.zshrc
# Clean, minimal prompt without user@host
# PROMPT='%F{blue}%~%f %# '
PROMPT='%F{blue}%~%f %|> '

# History
HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000
setopt HIST_IGNORE_DUPS HIST_IGNORE_SPACE SHARE_HISTORY

# Keybindings
bindkey -e

# Fast completion
autoload -Uz compinit && compinit -C

# Fix Ctrl+S freeze
stty -ixon

export PATH="$HOME/.local/bin:$PATH"

# Optional plugins
source ~/.zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh
source ~/.zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh

# bun completions
[ -s "/root/.bun/_bun" ] && source "/root/.bun/_bun"

# bun
export BUN_INSTALL="$HOME/.bun"
export PATH="$BUN_INSTALL/bin:$PATH"
