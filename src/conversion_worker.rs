use magick_rust::{MagickError, MagickWand};
use std::path::PathBuf;
use std::result::Result;
use tracing::info;
use walkdir::WalkDir;

use relm4::{ComponentSender, Worker};

#[derive(Debug)]
pub enum ConversionWorkerInputMsg {
    ConvertFolder(PathBuf, PathBuf),
}

#[derive(Debug)]
pub enum ConversionWorkerMsg {
    ConversionStarted(usize),
    ProgressUpdate(f64),
    ConversionComplete,
    ConversionFailed(String),
}

pub struct ConversionWorker;

impl Worker for ConversionWorker {
    type Init = ();
    type Input = ConversionWorkerInputMsg;
    type Output = ConversionWorkerMsg;

    fn init(_init: Self::Init, _sender: ComponentSender<Self>) -> Self {
        Self
    }

    fn update(&mut self, msg: ConversionWorkerInputMsg, sender: ComponentSender<Self>) {
        match msg {
            ConversionWorkerInputMsg::ConvertFolder(input_path, output_path) => {
                // Walk directory, find all heic files, convert them to jpg and update progress
                info!("Converting folder {:?}", input_path);
                let result = self.convert_folder(input_path, output_path, &sender);

                // Send the result of the conversion back
                match result {
                    Ok(_) => sender
                        .output(ConversionWorkerMsg::ConversionComplete)
                        .unwrap(),
                    Err(e) => sender
                        .output(ConversionWorkerMsg::ConversionFailed(e.to_string()))
                        .unwrap(),
                }
            }
        }
    }
}

impl ConversionWorker {
    fn convert_folder(
        &self,
        input_path: PathBuf,
        output_path: PathBuf,
        sender: &ComponentSender<Self>,
    ) -> Result<(), MagickError> {
        // Start the conversion
        info!("Converting folder {:?} to {:?}", input_path, output_path);

        // Use walkdir to find all heic files in the input directory
        let heic_files: Vec<PathBuf> = WalkDir::new(input_path)
            .follow_links(true)
            .same_file_system(false)
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file()
                    && path
                        .extension()
                        .map_or(false, |ext| ext.eq_ignore_ascii_case("heic"))
                {
                    Some(path.to_path_buf())
                } else {
                    None
                }
            })
            .collect();
        info!("Found {} heic files", heic_files.len());
        sender
            .output(ConversionWorkerMsg::ConversionStarted(heic_files.len()))
            .unwrap();

        // Convert each heic file to jpg
        for (index, heic_file) in heic_files.iter().enumerate() {
            info!("Converting file {:?}", heic_file);
            let output_file = output_path
                .join(heic_file.file_stem().unwrap())
                .with_extension("jpg");

            // Convert the file
            self.convert_heic_to_jpg(heic_file.to_path_buf(), output_file)?;

            // Update the progress
            sender
                .output(ConversionWorkerMsg::ProgressUpdate(
                    (index + 1) as f64 / heic_files.len() as f64,
                ))
                .unwrap();
        }
        info!("Conversion complete");
        Ok(())
    }

    fn convert_heic_to_jpg(
        &self,
        input_file: PathBuf,
        output_file: PathBuf,
    ) -> Result<(), MagickError> {
        // Create a MagickWand
        let mut wand = MagickWand::new();

        // Read the input file
        info!("Reading file {:?}", input_file);
        wand.read_image(input_file.to_str().unwrap())?;

        // Convert the image to jpg
        wand.set_image_format("jpg")?;
        info!("Converting to jpg");
        wand.write_image(output_file.to_str().unwrap())?;

        Ok(())
    }
}
