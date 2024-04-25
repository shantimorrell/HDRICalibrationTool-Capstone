use crate::pipeline::DEBUG;
use std::process::{Command, Stdio};
use std::io::Write;
use std::path::PathBuf;

/*  Utils.rs
 *
 *  Misc functions for quality of life
 * 
*/

/*
 *  start_piped_command
 *  An all-in-one funciton for calling a program which outputs the
 *  result to the STDOUT
 * 
 *  binary: String: The path and name of the binary to run
 *  args: &[String]: An array of arguments to pass into the binary
 * 
 *  Returns std::process::Output on success
 *          String on Failure
 */
pub fn start_piped_command(
    binary: PathBuf,
    args: Vec<&str>
) -> Result<std::process::Output, String>{
    // Init the command
    let command = Command::new(binary)
    .args(args);
    let binary_str: &str = binary.to_str().unwrap();

    let output = command.output().expect(format!("{binary_str} didn't finish properly!").as_str());

    // Output the data
    if output.status.success(){
        return Ok(output);
    }
    else{
        return Err(format!("{binary_str} failed!"));
    }
}

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
    binary: PathBuf,
    args: Vec<&str>,
    input_data: &std::process::Output
) -> Result<std::process::Output, String>{
    // Init the command
    let command = Command::new(binary)
    .args(args)
    .stdin(Stdio::piped());

    let str_binary : &str = binary.as_os_str().to_str().expect("Can't convert path to string!");
    // Put the command into a running state.
    let mut running = command.spawn().expect(format!("{str_binary} failed!").as_str());

    // Give the stdin data.
    let mut stdin = running.stdin.take().expect("Couldn't get stdin");
    std::thread::spawn(move || {
        stdin.write_all(&*input_data.stdout).expect("Couldn't input data into stdout");
    });

    let output = running.wait_with_output().expect(format!("{str_binary} didn't finish properly!").as_str());

    // Output the data
    if output.status.success(){
        return Ok(output);
    }
    else{
        return Err(format!("{str_binary} failed!"));
    }
}

/*
 *  return_output
 *  A macro for getting the return statement of a pipeline
 *  
 *  Returns std::process:Output on success
 *          String on Failure
 */
pub fn return_output(
    output : &Result<std::process::Output, String>,
    command_type : &str
) -> Result<std::process::Output, String>
{
    if output.is_ok() {
        // On success, return data
        if DEBUG {
            println!("Exit status of {:?} is {:?}", command_type, output.unwrap().status.code());
        }
        Ok(output.unwrap())
    } else {
        Err(output.unwrap_err())
    }
}