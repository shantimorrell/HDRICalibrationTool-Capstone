use crate::pipeline::DEBUG;
use std::fs::File;
use std::process::Command;
use std::process::Stdio;

use super::ConfigSettings;

// Crops a fisheye view HDR image to a square which circumscribes the circular fisheye view.
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the cropped HDR image will be saved.
// diameter:
//      the fisheye view diameter in pixels
// xleft:
//      The x-coordinate of the bottom left corner of the circumscribed square
//      of the fisheye view (in pixels)
// ydown:
//      The y-coordinate of the bottom left corner of the circumscribed square
//      of the fisheye view (in pixels)
pub fn crop(
    config_settings: &ConfigSettings,
    input_file: String,
    output_file: String,
    diameter: String,
    xleft: String,
    ydown: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("crop() was called with parameters:");
        println!("\tdiameter: {diameter}");
        println!("\txleft: {xleft}");
        println!("\tydown: {ydown}");
    }

    // Create a new command for pcompos
    let mut command = Command::new(config_settings.radiance_path.to_string() + "pcompos");

    // Add arguments to pcompos command
    command.args([
        "-x",
        diameter.as_str(),
        "-y",
        diameter.as_str(),
        input_file.as_str(),
        format!("-{xleft}").as_str(),
        format!("-{ydown}").as_str(),
    ]);

    // Run the command, and get the output
    let output = command.output().expect("Crop failed!");

    if DEBUG {
        println!("\nCrop command exit status: {:?}\n", output.status);
    }

    assert!(output.status.success());

    return Ok(output);
}
