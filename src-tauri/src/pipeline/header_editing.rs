use crate::pipeline::DEBUG;
use crate::pipeline_utils::piped_command;
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
    input_data: &std::process::Output,
    vertical_angle: String,
    horizontal_angle: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("header_editing() was called with parameters:\n\tvertical_angle: {vertical_angle}\n\thorizontal_angle: {horizontal_angle}");
    }

    let output: Result<std::process::Output, String> = piped_command(
        config_settings.radiance_path.join("getinfo"),
        vec![
            "-a",
            format!("VIEW= -vta -vv {} -vh {}", vertical_angle, horizontal_angle).as_str(),
        ],
        input_data
    );

    if DEBUG {
        println!(
            "\nHeader editing command exit status: {:?}\n",
            output.as_ref().unwrap().status.code()
        );
    }

    return output;
}