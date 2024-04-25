use crate::pipeline::DEBUG;
use crate::pipeline_utils::piped_command;
use crate::pipeline_utils::return_output;

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
    input_data: &std::process::Output,
    photometric_adjustment: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("photometric_adjustment() was called with parameters:\n\t photometric_adjustment: {photometric_adjustment}");
    }

    // Run photometric Adjustments
    let output: Result<std::process::Output, String> = piped_command(
        config_settings.radiance_path.join("pcomb"),
        vec![
            "-h",
            "-f",
            photometric_adjustment.as_str(),
        ],
        input_data
    );

    return return_output(&output, "Photometric Adjustment");
}