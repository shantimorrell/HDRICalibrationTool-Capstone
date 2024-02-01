use std::process::{Command, Stdio};
use std::io::Write;

/*  Utils.rs
 *
 *  Misc functions for quality of life
 * 
*/

/*
 *  piped_command
 *  An all-in-one funciton for calling a program which requires
 *  an STDIN from a previous program, then gives the STDOUT
 * 
 *  binary: String: The path and name of the binary to run
 *  args: &[String]: An array of arguments to pass into the binary
 *  stdin: std::process::Output: The output of the previous program
 * 
 *  Returns std::process::Output on success
 *          String on Failure
 */
pub fn piped_command(
    binary: String,
    args: Vec<&str>,
    input_data: std::process::Output
) -> Result<std::process::Output, String>{
    // Init the command
    let command = Command::new(binary)
    .args(args)
    .stdin(Stdio::piped());

    // Put the command into a running state.
    let mut running = command.spawn().expect(format!("{binary} failed!").as_str());

    // Give the stdin data.
    let mut stdin = running.stdin.take().expect("Couldn't get stdin");
    std::thread::spawn(move || {
        stdin.write_all(&*input_data.stdout).expect("Couldn't input data into stdout");
    });

    let output = running.wait_with_output().expect(format!("{binary} didn't finish properly!").as_str());

    // Output the data
    if output.status.success(){
        return Ok(output);
    }
    else{
        return Err(format!("{binary} failed!"));
    }
}