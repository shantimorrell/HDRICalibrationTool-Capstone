use crate::pipeline::DEBUG;
use crate::pipeline_utils::{
    piped_command,
    return_output
};

use super::ConfigSettings;

// Corrects for the vignetting effect of an HDR image using pcomb.
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the HDR image with nullified
//      exposure value will be saved.
// vignetting_correction_cal:
//      a string for the vignetting correction calibration file
pub fn vignetting_effect_correction(
    config_settings: &ConfigSettings,
    input_data: &std::process::Output,
    vignetting_correction_cal: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("vignetting_effect_correction() was called with parameters:");
        println!("\tvignetting_correction_cal: {vignetting_correction_cal}");
    }

    // Run pcomb
    let output : Result<std::process::Output, String> = piped_command(
        config_settings.radiance_path.join("pcomb"),
        vec![
            "-f",
            vignetting_correction_cal.as_str(),
        ],
        input_data
    );

    return return_output(&output, "Vignetting Correction");
}
