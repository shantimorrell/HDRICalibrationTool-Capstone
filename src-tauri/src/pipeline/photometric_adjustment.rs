use crate::pipeline::DEBUG;
use std::process::Command;
use std::process::Stdio;
use std::fs::File;
use std::io::Write;

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
    input_data: std::process::Output,
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
    ]);

    // Set the stdin as the input data
    command.stdin(Stdio::piped());

    // Run the command, waiting for stdin.
    let running = command.spawn().expect("Photometric adjustment failed");

    // Give stdin data.
    let mut stdin = running.stdin.take().expect("Couldn't get stdin");
    std::thread::spawn(move || {
        stdin.write_all(&*input_data.stdout).expect("Issue getting previous stdout");
    });

    let output = running.wait_with_output().expect("Process didn't run properly");

    if DEBUG {
        println!(
            "\nPhotometric adjustment command exit status: {:?}\n",
            output.status
        );
    }
    
    assert!(output.status.success());

    return Ok(output);
}