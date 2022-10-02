use clap::{Args, Parser, Subcommand};
use eyre::Result;
use fern::{colors::ColoredLevelConfig, Dispatch, InitError};
use image::{io::Reader, GenericImageView};
use log::{info, warn};
use std::{fs::File, io::Write, path::PathBuf};

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Convert an image into a gas giant
    Convert(Convert),
    /// Check whether an image is a valid gas giant
    Valid(Valid),
}

#[derive(Args, Debug)]
struct Convert {
    /// Path to input image. Relative to current working directory
    #[arg(short, long, default_value = "ggg-input.png")]
    input: PathBuf,
    /// Path to output json. Relative to current working directory
    #[arg(short, long, default_value = "ggg-output.json")]
    output: PathBuf,
    /// Use a different x-position than 0 if your input's width is > 1
    #[arg(long, default_value_t = 0u32)]
    pos: u32,
}

#[derive(Args, Debug)]
struct Valid {
    /// Path to input image. Relative to current working directory
    #[arg(short, long, default_value = "ggg-input.png")]
    input: PathBuf,
}

fn convert(args: Convert) -> Result<()> {
    let input = match Reader::open(&args.input) {
        Ok(ip) => match ip.decode() {
            Ok(di) => di,
            Err(e) => panic!(
                "failed to decode file: {}, here's a 'detailed' description of the error: {}",
                args.input.to_string_lossy(),
                e.to_string().to_lowercase()
            ),
        },
        Err(_) => panic!(
            "failed to read file: {}, maybe you misspelt it?",
            args.input.to_string_lossy()
        ),
    };

    if input.width() > 1u32 {
        warn!(
            "{} width is above 1! using pixels with width: {} (pos)",
            &args.input.to_string_lossy(),
            &args.pos
        )
    }

    let mut output = String::new();
    output.push_str("\"Colors\":[\n");

    for pixel in input.pixels() {
        if pixel.0 != args.pos.clone() {
            continue;
        }

        output.push_str(
            format!(
                "\"RGBA({:.3}, {:.3}, {:.3}, {:.3})\"{}{}",
                pixel.2[0usize] as f32 / 255.0f32,
                pixel.2[1usize] as f32 / 255.0f32,
                pixel.2[2usize] as f32 / 255.0f32,
                pixel.2[3usize] as f32 / 255.0f32,
                match pixel.1 == input.height() - 1u32 {
                    true => {
                        "],"
                    }
                    false => ",",
                },
                match (pixel.1 + 1u32) % 10u32 == 0u32 {
                    true => "\n",
                    false => "",
                },
            )
            .as_str(),
        )
    }

    let mut file = File::create(format!("{}", args.output.to_string_lossy()))?;
    writeln!(file, "{}", output)?;

    Ok(())
}

fn valid(args: Valid) -> Result<()> {
    let input = match Reader::open(&args.input) {
        Ok(ip) => match ip.decode() {
            Ok(di) => di,
            Err(e) => panic!(
                "failed to decode file: {}, here's a 'detailed' description of the error: {}",
                args.input.to_string_lossy(),
                e.to_string().to_lowercase(),
            ),
        },
        Err(_) => panic!(
            "failed to read file: {}, maybe you misspelt it?",
            args.input.to_string_lossy(),
        ),
    };

    if input.height() > 16384u32 {
        info!(
            "{} is not a valid gas giant! gas giants cannot have above 16384 bands (your input's height is: {})",
            args.input.to_string_lossy(),
            input.height(),
        )
    } else {
        info!(
            "{} is (most likely) a valid gas giant.",
            args.input.to_string_lossy(),
        )
    }

    Ok(())
}

fn setup_logger() -> Result<(), InitError> {
    let colors = ColoredLevelConfig::default();

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: '{}'",
                colors.color(record.level()).to_string().to_lowercase(),
                message
            ))
        })
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() -> Result<()> {
    setup_logger()?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Convert(Convert { input, output, pos })) => convert(Convert {
            input: input.clone(),
            output: output.clone(),
            pos: pos.clone(),
        }),
        Some(Commands::Valid(Valid { input })) => valid(Valid {
            input: input.clone(),
        }),
        None => convert(Convert {
            input: PathBuf::from("ggg-input.png"),
            output: PathBuf::from("ggg-output.json"),
            pos: 0u32,
        }),
    }
}
