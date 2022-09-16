# computer dashboard

A [glances-like](https://github.com/nicolargo/glances) dashboard for a linux computer
![Dashboard screenshot](/assets/comp-dash.png)

# How to run
1. Clone the repo
2. Run `cargo run` from root directory.

To run with debug level log, use: `LOG_LEVEL=debug cargo run`.
Logs will be written to `output.log` file.

# Limitations
Only runs on linux, due to the limitations of the libraries used.
