use crate::pipeline::DEBUG;
use crate::pipeline_utils::piped_command;
use crate::pipeline_utils::return_output;

use super::ConfigSettings;


// Neutral Density Filter
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the cropped HDR image will be saved.
// neutral_density:
//      A string for the neutral density file

pub fn neutral_density(
    config_settings: &ConfigSettings,
    input_data: &std::process::Output,
    neutral_density: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("neutral_density() was called with parameters:\n\t neutral_density: {neutral_density}");
    }

    // Run neutral density, with image inputs, and get the STDOUT.
    let output = piped_command(
        config_settings.radiance_path.join("pcomb"),
        vec![
            "-f",
            neutral_density.as_str()
        ],
        input_data
    );

    return return_output(&output, "Neutral Density");
}