use log::debug;

use std::process::{ExitStatus, Stdio};

use tokio::io::{AsyncReadExt, AsyncWriteExt as _};
use tokio::process::{Child as TChild, ChildStdin as TChildStdin, Command as TCommand};

pub async fn eflint_to_json(input: String) -> Result<String, String> {
    let bin_path = "./bin/eflint-to-json";

    debug!("Using compiler at: '{}'", bin_path);

    // Alrighty well open a handle to the compiler
    debug!("Spawning compiler '{}'", bin_path);
    let mut cmd: TCommand = TCommand::new(bin_path);
    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    let mut handle: TChild = match cmd.spawn() {
        Ok(handle) => handle,
        Err(err) => {
            return Err(format!("{:?}", err));
        }
    };

    // Feed the input to the compiler, analyzing for `#input(...)` and `#require(...)`
    debug!("Reading input to child process...");
    let mut stdin: TChildStdin = handle.stdin.take().unwrap();
    match stdin.write_all(input.as_bytes()).await {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Could not write to stdin: {:?}", err));
        }
    }
    drop(stdin);

    // Wait until the process is finished
    debug!("Waiting for child process to complete...");
    let status: ExitStatus = match handle.wait().await {
        Ok(status) => status,
        Err(err) => {
            return Err(format!("{:?}", err));
        }
    };
    if !status.success() {
        let mut buf: String = String::new();
        match handle.stderr.take().unwrap().read_to_string(&mut buf).await {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("<failed to read err stream:{:?}>", err));
            }
        };

        return Err(format!("{:?}", buf));
    }

    // Alrighty, now it's time to stream the output of the child to the output file
    debug!("Writing child process output to given output...");
    let mut ret: String = String::new();
    match handle.stdout.take().unwrap().read_to_string(&mut ret).await {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("<failed to read output stream:{:?}>", err));
        }
    };

    Ok(ret)
}
