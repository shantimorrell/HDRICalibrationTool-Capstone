use crate::pipeline::DEBUG;
use std::process::Command;

use super::ConfigSettings;

// Nullifies the exposure value of an HDR image using ra_xyze.
// config_settings:
//    contains config settings - used for path to radiance and temp directory
// input_file:
//    the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//    a string for the path and filename where the HDR image with nullified
//    exposure value will be saved.
pub fn nullify_exposure_value(
    config_settings: &ConfigSettings,
    input_file: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("nullify_exposure_value was called!");
    }

    // Create a new command for ra_xyze
    let mut command = Command::new(config_settings.radiance_path.to_string() + "ra_xyze");

    // Add arguments to ra_xyze command
    command.args(["-r", "-o", input_file.as_str()]);

    // Run the command
    let output = command.output().expect("Could not successfully nulify exposure values!");

    if DEBUG {
        println!(
            "\nNullication of exposure value command exit status: {:?}\n",
            output.status.code()
        );
    }

    if !output.status.success(){
        return Err("Issue with Nullification exposure".into());
    }

    return Ok(output);
}
