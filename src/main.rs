use clap::Parser;
use clap::Subcommand;
use eyre::bail;
use eyre::Context;
use eyre::Result;
use image::io::Reader;
use image::GenericImageView;
use owo_colors::OwoColorize;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write as _;
use std::path::PathBuf;
use zip::write::FileOptions;
use zip::ZipArchive;
use zip::ZipWriter;

#[derive(Parser)]
struct Cli {
    /// Image to check/convert
    image: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert an image to a gas giant
    Convert {
        /// Path to the ubox containing your gas giant
        old_giant: PathBuf,
        /// Filename of the newly created body, don't pass to overwrite the original
        new_giant: Option<String>,
        /// Column of your image to use if its width is not 1
        #[arg(default_value_t = 0u32)]
        pos: u32,
    },
    /// Check if an image is a valid gas giant
    Check,
    /// ?
    Print,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let image = Reader::open(&cli.image)?.decode()?;
    let filename = cli.image.file_name().unwrap().to_string_lossy();

    match cli.command {
        Commands::Convert {
            old_giant,
            new_giant,
            pos,
        } => {
            let mut ubox =
                ZipArchive::new(File::open(&old_giant).wrap_err("ubox provided does not exist!")?)
                    .wrap_err("ubox provided is not a renamed zip!")?;
            let body = ubox.by_name("body.json")?;
            let mut json = serde_json::from_reader::<_, Value>(body)
                .wrap_err("body.json is not valid json!")?;
            let components = json
                .as_object_mut()
                .expect("Not an object")
                .get_mut("Components")
                .expect("Components is missing")
                .as_array_mut()
                .expect("Components is not an array");

            let mut found = false;
            for component in components {
                if component.get("$type").expect("$type is missing") == "AppearanceComponent" {
                    found = true;

                    let colors = component
                        .get_mut("GasGiant")
                        .expect("Not a gas giant")
                        .as_object_mut()
                        .expect("Not an object")
                        .get_mut("Colors")
                        .expect("Colors is missing")
                        .as_array_mut()
                        .expect("Colors is not an array");

                    colors.clear();

                    for pixel in image.pixels().filter(|p| p.0 == pos) {
                        colors.push(Value::String(format!(
                            "RGBA({:.3}, {:.3}, {:.3}, {:.3})",
                            pixel.2[0usize] as f32 / 255.0f32,
                            pixel.2[1usize] as f32 / 255.0f32,
                            pixel.2[2usize] as f32 / 255.0f32,
                            pixel.2[3usize] as f32 / 255.0f32,
                        )))
                    }

                    break;
                }
            }

            // early return if we could not find this
            if !found {
                bail!("Failed to find AppearanceComponent!");
            }

            let overwrite = new_giant.is_none();
            let new_giant = new_giant.unwrap_or("TEMPUBOX.ubox".to_owned());
            File::create(&new_giant)?;

            let mut writer = ZipWriter::new(File::options().write(true).open(&new_giant)?);
            for filename in
                // fuck this shit
                ZipArchive::new(
                    File::open(&old_giant).wrap_err("ubox provided does not exist!")?,
                )
                .wrap_err("ubox provided is not a renamed zip!")?
                .file_names()
            {
                if filename != "body.json" {
                    writer.raw_copy_file(ubox.by_name(filename)?)?;
                }
            }
            writer.start_file("body.json", FileOptions::default())?;
            writer.write_all(format!("{json}").as_bytes())?;
            writer.finish()?;

            if overwrite {
                fs::copy(&new_giant, old_giant)?;
                // because it's "TEMPUBOX.ubox"
                fs::remove_file(new_giant)?;
            }

            writeln!(std::fs::File::create("a")?, "{}", json)?;
        }
        Commands::Check => {
            // lol what's the point

            if image.height() > 16384 {
                println!("{} is not a valid gas giant! They cannot have above 16384 bands (your image would create a giant with {} bands).", filename.bold(), image.height().bold());
            } else if image.width() != 1 {
                println!(
                    "{} would (most likely) be a valid gas giant.",
                    filename.bold()
                )
            }
        }
        Commands::Print => println!(
            "{0:128}\n{1:128}\n{2:128}\n{1:128}\n{0:128}",
            " ".on_blue(),
            " ".on_purple(),
            " ".on_white(),
        ),
    }

    Ok(())
}
