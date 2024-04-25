use crate::pipeline::DEBUG;
use crate::pipeline_utils::start_piped_command;
use crate::pipeline_utils::return_output;

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

    // Create a new command for ra_xyze and run it
    let output : Result<std::process::Output, String> = start_piped_command(
        config_settings.radiance_path.join("ra_xyze"),
        vec![
            "-r",
            "-o",
            input_file.as_str()
        ]
    );

    return return_output(&output, "Nullify Exposure");
}
