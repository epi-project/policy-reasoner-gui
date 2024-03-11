//  BUILD.rs
//    by Lut99
//
//  Created:
//    09 Feb 2024, 14:49:56
//  Last edited:
//    11 Mar 2024, 12:01:58
//  Auto updated?
//    Yes
//
//  Description:
//!   Build script to compile dependencies (client, eflint-to-json) of the
//!   backend server.
//

use std::path::Path;
use std::process::{Command, Output};

use error_trace::trace;


/***** CONSTANTS *****/
/// The command to run NPM package install.
pub const NPM_INSTALL_COMMAND: [&'static str; 2] = ["npm", "i"];
/// The command to run parcel's build.
pub const NPM_BUILD_COMMAND: [&'static str; 3] = ["npx", "parcel", "build"];





/***** HELPER FUNCTIONS *****/
/// Runs a given command in a given working directory.
///
/// # Arguments
/// - `cmd`: A list of strings that denotes the command to run.
/// - `cwd`: The path to the working directory to use.
///
/// # Panics
/// This function may panic if the command failed to run for any reason.
fn run_command<'s>(cmd: impl IntoIterator<Item = &'s str>, cwd: impl AsRef<Path>) {
    let mut cmd = cmd.into_iter();
    let cwd: &Path = cwd.as_ref();

    // Assert we have at least one thing
    let exec: &str = match cmd.next() {
        Some(exec) => exec,
        None => panic!("Got empty command list"),
    };

    // Build the command
    let mut command: Command = Command::new(exec);
    command.args(cmd);
    command.current_dir(cwd);

    // Run it
    let output: Output = match command.output() {
        Ok(out) => out,
        Err(err) => panic!("{}", trace!(("Failed to run command '{command:?}'"), err)),
    };
    if !output.status.success() {
        panic!(
            "Failed to run command '{command:?}'\n\nstdout:\n{}\n{}\n{}\n\nstderr:\n{}\n{}\n{}\n",
            (0..80).map(|_| '-').collect::<String>(),
            String::from_utf8_lossy(&output.stdout),
            (0..80).map(|_| '-').collect::<String>(),
            (0..80).map(|_| '-').collect::<String>(),
            String::from_utf8_lossy(&output.stderr),
            (0..80).map(|_| '-').collect::<String>()
        );
    }
}





/***** ENTRYPOINT *****/
fn main() {
    // npm i # alleen als er sinds de laatste keer dependency updates zijn gedaan maar kan geen kwaad om het altijd te doen
    // npx parcel build --dist-dir [dist directory dus bijv ../client-build oid]

    // Mark the triggers for this build script
    println!("cargo:rerun-if-changed={}", concat!(env!("CARGO_MANIFEST_DIR"), "/client/src"));
    println!("cargo:rerun-if-changed={}", concat!(env!("CARGO_MANIFEST_DIR"), "/client/.parcelrc"));
    println!("cargo:rerun-if-changed={}", concat!(env!("CARGO_MANIFEST_DIR"), "/client/package-lock.json"));
    println!("cargo:rerun-if-changed={}", concat!(env!("CARGO_MANIFEST_DIR"), "/client/package.json"));

    // OK, run the NPM install command
    let build_dir: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/client");
    run_command(NPM_INSTALL_COMMAND, build_dir);

    // Build the full args for the build command, then run that
    run_command(NPM_BUILD_COMMAND.into_iter().chain(["--dist-dir", concat!(env!("CARGO_MANIFEST_DIR"), "/clientbuild")]), build_dir);

    // Done
}
