<p align="center">
  <h1 align="center">tash</h1>
  <p align="center">
    <a href="https://github.com/dhth/tash/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/tash/main.yml?style=flat-square"></a>
    <a href="https://crates.io/crates/tash"><img alt="crates.io" src="https://img.shields.io/crates/v/tash?style=flat-square"></a>
    <a href="https://github.com/dhth/tash/releases/latest"><img alt="Latest Release" src="https://img.shields.io/github/release/dhth/tash.svg?style=flat-square"></a>
    <a href="https://github.com/dhth/tash/releases"><img alt="Commits Since Latest Release" src="https://img.shields.io/github/commits-since/dhth/tash/latest?style=flat-square"></a>
  </p>
</p>

✨ Overview
---

`tomo` is a no-frills pomodoro progress indicator intended for tmux and similar
terminal multiplexers.

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
      --finished-msg <STRING>    Message to show when timer is finished [default: done]
      --break-msg <STRING>       Message to show when on a break [default: \o/]
  -h, --help                     Print help
```

### Changing the appearance of the progress bar

```bash
tomo -l='[ ' -r=' ]'
# [ ▪▪▪▪▪▫▫▫▫▫ ]

tomo -p='⣀' -c='⣿' -n=20
# ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣀⣀⣀⣀⣀ 
```

### Start tracking with time already elapsed

```bash
tomo start --elapsed-mins 10
```

### Displaying progress bar in tmux's status bar

Add the following to your tmux config (or modify it accordingly).

```
set -g status-right "#(tomo)"
```
