use crate::pipeline::DEBUG;
use std::process::{Command, Stdio};
use std::fs::File;
use std::io::Write;
// use regex::Regex;

use super::ConfigSettings;


// Header Editing
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the cropped HDR image will be saved.
// vertical_angle:
//      The fov, in degrees, of the image vertically. Found within the camera settings.
// horizontal_angle:
//      The fov, in degrees, of the image horizontally. Found within the camera settings.

pub fn header_editing(
    config_settings: &ConfigSettings,
    input_data: std::process::Output,
    vertical_angle: String,
    horizontal_angle: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("header_editing() was called with parameters:\n\tvertical_angle: {vertical_angle}\n\thorizontal_angle: {horizontal_angle}");
    }

    return Err("Hai!".to_string());

    /*
    TODO: Looking into using regex instead of sed, so this can run on Windows without a problem.
    // Get the header info
    let mut command_header_get = Command::new(config_settings.radiance_path.to_string() + "getinfo");

    // Add arguments
    command_header_get.args([
        input_file,
    ]);

    // Run the command
    command_header_get.status.unwrap();

    // And remove the line containing the VIEW angles
    let re = Regex::new(r".*VIEW=.*");
    let freshFile = re.replace_all(String::from_utf8_lossy(&command.stdout));

    // Modify the input file to show these changes
    let fileClearedInput = File::create(&input_file).unwrap();
    fileClearedInput.write_all(freshFile);
    */

    // Apply the new header
    // let mut command = Command::new(config_settings.radiance_path.to_string() + "getinfo");

    // // Add arguments
    // command.args([
    //     "-a",
    //     format!("VIEW= -vta -vv {} -vh {}", vertical_angle, horizontal_angle).as_str(),
    // ]);

    // // Set up stdin
    // command.stdin(Stdio::piped());

    // let processing = command.spawn().expect("Could not spawn process!");

    // // Set up piping of the input file
    // let mut stdin = processing.stdin.take().expect("Failed to init stdin.");
    // std::thread::spawn(move || {
    //     stdin.write_all(input_data.stdout.as_slice());
    // });

    // // Run the command, and get the output.
    // let output = command.output().expect("Header Editing failed");

    // // Run the command
    // let status = command.status();

    // if DEBUG {
    //     println!(
    //         "\nHeader editing command exit status: {:?}\n",
    //         status
    //     );
    // }

    // assert!(output.status.success());

    // return Ok(output);
}