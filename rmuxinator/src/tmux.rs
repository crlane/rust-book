#[macro_use]
extern crate log;

use std::process::Command;


fn tmux(args: Vec<&str>) -> Command {
    let mut cmd = Command::new("tmux");
    cmd.args(args);
    trace!("Executing: {:?}", cmd);
    cmd
}
