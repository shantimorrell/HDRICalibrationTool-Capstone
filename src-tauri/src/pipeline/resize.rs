use crate::pipeline::DEBUG;
use crate::pipeline_utils::{
    piped_command,
    return_output
};

use super::ConfigSettings;

// Resizes an HDR image to the target x and y resolution.
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the cropped HDR image will be saved.
// xdim:
//      The x-dimensional resolution to resize the HDR image to (in pixels)
// ydim:
//      The y-dimensional resolution to resize the HDR image to (in pixels)
pub fn resize(
    config_settings: &ConfigSettings,
    input_data: &std::process::Output,
    xdim: String,
    ydim: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("resize() was called with parameters:");
        println!("\txdim: {xdim}");
        println!("\tydim: {ydim}");
    }

    // Run pfilt
    let output: Result<std::process::Output, String> = piped_command(
        config_settings.radiance_path.join("pfilt"),
        vec![
            "-1",
            "-x",
            xdim.as_str(),
            "-y",
            ydim.as_str()
        ],
        input_data
    );

    return return_output(&output, "Resize");
}