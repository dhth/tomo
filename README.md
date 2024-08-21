# tomo

✨ Overview
---

`tomo` is a no-frills pomodoro progress indicator intended for tmux and similar
terminal multiplexers.

It is a port of [pomm](https://github.com/dhth/pomm).

⚡️ Usage
---

### Basic Usage

```text
Usage: tomo [OPTIONS] [COMMAND]

Commands:
  start  Start a pomodoro timer
  stop   Stop timer
  break  Start a break
  help   Print this message or the help of the given subcommand(s)

Options:
  -p, --pending-block <STRING>   String to represent a "pending" block in the progress bar [default: ▫]
  -c, --complete-block <STRING>  String to represent a "complete" block in the progress bar [default: ▪]
  -l, --left-pad <STRING>        String to pad the output with on the LHS [default: " "]
  -r, --right-pad <STRING>       String to pad the output with on the RHS [default: " "]
  -d, --delimiter <STRING>       Delimiter between progress bar chunks [default: ]
  -n, --num-blocks <NUM>         Number of blocks to show in progress bar [default: 10]
  -h, --help                     Print help
```

### Changing the appearance of the progress bar

```bash
tomo -l='[ ' -r=' ]'
# [ ▪▪▪▪▪▫▫▫▫▫ ]

tomo -p='⣀' -c='⣿' -n=20
# ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣀⣀⣀⣀⣀ 
```

### Displaying progress bar in tmux's status bar

Add the following to your tmux config (or modify it accordingly).

```
set -g status-right "#(tomo)"
```

### Start tracking with time already elapsed

```bash
tomo start --elapsed-mins 10
```
