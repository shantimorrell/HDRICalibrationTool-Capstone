use crate::pipeline::DEBUG;
use std::process::Command;
use std::process::Stdio;
use std::fs::File;

use super::ConfigSettings;


// Photometric Adjustments
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the cropped HDR image will be saved.
// photometric_adjustment:
//      A string for the photometric adjustment file

pub fn photometric_adjustment(
    config_settings: &ConfigSettings,
    input_file: String,
    output_file: String,
    photometric_adjustment: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("photometric_adjustment() was called with parameters:\n\t photometric_adjustment: {photometric_adjustment}");
    }

    // Command to run
    let mut command = Command::new(config_settings.radiance_path.to_string() + "pcomb");

    // Add arguments
    command.args([
        "-h",
        "-f",
        photometric_adjustment.as_str(),
        input_file.as_str(),
    ]);

    // Run the command, and get the output.
    let output = command.output().expect("Photometric adjustment failed");

    if DEBUG {
        println!(
            "\nPhotometric adjustment command exit status: {:?}\n",
            output.status
        );
    }
    
    assert!(output.status.success());

    return Ok(output);
}