use crate::pipeline::DEBUG;
use crate::pipeline_utils::{
    piped_command,
    return_output
};

use super::ConfigSettings;

// Crops a fisheye view HDR image to a square which circumscribes the circular fisheye view.
// config_settings:
//      contains config settings - used for path to radiance and temp directory
// input_file:
//      the path to the input HDR image. Input image must be in .hdr format.
// output_file:
//      a string for the path and filename where the cropped HDR image will be saved.
// diameter:
//      the fisheye view diameter in pixels
// xleft:
//      The x-coordinate of the bottom left corner of the circumscribed square
//      of the fisheye view (in pixels)
// ydown:
//      The y-coordinate of the bottom left corner of the circumscribed square
//      of the fisheye view (in pixels)
pub fn crop(
    config_settings: &ConfigSettings,
    input_data: &std::process::Output,
    diameter: String,
    xleft: String,
    ydown: String,
) -> Result<std::process::Output, String> {
    if DEBUG {
        println!("crop() was called with parameters:");
        println!("\tdiameter: {diameter}");
        println!("\txleft: {xleft}");
        println!("\tydown: {ydown}");
    }

    // Run the command
    let output : Result<std::process::Output, String> = piped_command(
        config_settings.radiance_path.join("pcompos"),
        vec![
            "-x",
            diameter.as_str(),
            "-y",
            diameter.as_str(),
            format!("-{xleft}").as_str(),
            format!("-{ydown}").as_str(),
        ],
        input_data
    );

    return return_output(&output, "Crop");
}
