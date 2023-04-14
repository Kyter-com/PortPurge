# To Do

- Support stderror for the shell command.
- Support Windows using the std::env::consts::OS variable.
  - Support WSL on Windows.
- Break out the port process finding function to a separate file that returns the pid and potentially other info.
- Remove clap dependency and use std::env::args() instead.
