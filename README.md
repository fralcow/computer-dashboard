# computer dashboard

A glances-like dashboard for a linux computer

# How to run
1. Clone the repo
2. Run `RUST_LOG=debug cargo run 2> {another_tty}` from root directory.
Logs are outputted to stderr that is then piped to another tty that you should open before running the command.
