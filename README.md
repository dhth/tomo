# tomo

✨ Overview
---

`tomo` is a no-frills pomodoro progress indicator intended for tmux and similar
terminal multiplexers.

It is a port of [pomm](https://github.com/dhth/pomm).

⚡️ Usage
---

### Basic Usage

```bash
tomo start              # to start tracking
tomo                    # to show pomodoro progress, eg. ` ▪▪▪▪▪▪▪▪▫▫ `
tomo break              # to take a break (running "tomo" will output ` break `)
tomo stop               # to stop tracking (running "tomo" will show no output)

tomo \                  # `[ ▪▪▪▪▪▪▪▪▫▫ ]`
    --left-pad='[ ' \
    --right-pad=' ]'
```

### Displaying progress bar in tmux's status bar

Add the following to your tmux config (or modify it accordingly).

```
set -g status-right "#(tomo)"
```
