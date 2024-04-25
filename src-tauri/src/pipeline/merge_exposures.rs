use crate::pipeline::DEBUG;
use mime;
use mime_guess;
use std::{
    path::Path,
    process::{Command, ExitStatus},
};

use super::ConfigSettings;

// const RAW: bool = true;

// Merges multiple LDR images into an HDR image using hdrgen.
// input_images:
//    vector of the paths to the input images. Input images must be in .JPG format.
// response_function:
//    string for the path to the camera response function, must be a .rsp file
// output_path:
//    a string for the path and filename where the resulting HDR image will be saved.
#[tauri::command]
pub fn merge_exposures(
    config_settings: &ConfigSettings,
    input_images: Vec<String>,
    response_function: String,
    output_path: String,
) -> Result<String, String> {
    if DEBUG {
        println!("merge_exposures Tauri command was called!");
    }

    // Check whether images are in jpeg format
    let first_image_ext = Path::new(&input_images[0]).extension().unwrap_or_default();
    let first_image_type: Option<mime::Mime> = mime_guess::from_path(input_images[0].as_str()).first();
    // Check to make sure it's either a jpg or raw
    let raw_images: bool = first_image_type != Some(mime::IMAGE_JPEG);

    if DEBUG {
        println!(
            "\n\nMerge exposures running in {} MODE...\n\n",
            if raw_images { "RAW" } else { "JPG" }
        );
    }

    let mut command: &mut Command;
    if raw_images {
        // Run RAW2HDR
        command = Command::new(
            config_settings.raw2hdr_path.join("raw2hdr")
        )
        .args([
            "-o", // Output Path for HDR image
            format!("{}", output_path).as_str(),
        ]);

        // Add input raw LDR images as args
        for input_image in input_images {
            command.arg(format!("{}", input_image));
        }
    } else {
        // Create a new command for hdrgen
        command = Command::new(
            config_settings.hdrgen_path.join("hdrgen")
        )
        .args([
            // Add output path for HDR image
            "-o",
            format!("{}", output_path).as_str(),
            // Add camera response function
            "-r",
            format!("{}", response_function).as_str(),
            // Extra flags to disable auto-adjustments
            "-a",
            "-e",
            "-f",
            "-g"
        ]);

        // Add input LDR images as args
        for input_image in input_images {
            command.arg(format!("{}", input_image));
        }
    }

    // Run the command
    let status: Result<ExitStatus, std::io::Error> = command.status();

    if DEBUG {
        println!("\nCommand exit status: {:?}\n", status);
    }

    // Return a Result object to indicate whether hdrgen command was successful
    if !status.is_ok() || !status.unwrap_or(ExitStatus::default()).success() {
        // On error, return an error message
        Err(format!(
            "Error, non-zero exit status. {} command failed.",
            if raw_images { "raw2hdr" } else { "hdrgen" }
        )
        .into())
    } else {
        // On success, return output path of HDR image
        Ok(output_path.into())
    }
}
