use crate::pipeline::DEBUG;
use crate::pipeline_utils::{
    piped_command,
    return_output
};

use super::ConfigSettings;

// Applies projection adjustment for the fisheye lens to an HDR image using pcomb.
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the HDR image with nullified
//      exposure value will be saved.
// fisheye_correction_cal:
//      a string for the fisheye correction calibration file
pub fn projection_adjustment(
    config_settings: &ConfigSettings,
    input_data: &std::process::Output,
    fisheye_correction_cal: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("projection_adjustment() was called with parameters:");
        println!("\tfisheye_correction_cal: {fisheye_correction_cal}");
    }

    // Run pcomb
    let output : Result<std::process::Output, String> = piped_command(
        config_settings.radiance_path.join("pcomb"),
        vec![
            "-f",
            fisheye_correction_cal.as_str(),
        ],
        input_data
    );

    return_output(&output, "Projection Adjustments")
}
