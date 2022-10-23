use clap::{Parser, Subcommand};
use env_logger::Builder;
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::ProgressBar;
use log::{info, warn, LevelFilter};
use rand::{thread_rng, Rng};

use std::fs;
use std::io::Write;
use std::num::IntErrorKind;
use std::num::ParseIntError;

use wallpaper;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate a new Sierpiński triangle
    Generate {
        /// Width of the image (In pixels)
        #[clap(short, long)]
        width: u32,

        /// Height of the image (In pixels)
        #[clap(short, long)]
        height: u32,

        /// Number of dots to draw on the image
        #[clap(short, long)]
        dots: u64,

        /// The path of the output image
        #[clap(short, long, name = "FILE")]
        output: Option<String>,

        /// The color of the pixels being placed (In hex format)
        #[clap(short, long)]
        color: Option<String>,

        /// Set the generated image as wallpaper
        #[clap(long)]
        wallpaper: bool,
    },
}

fn main() {
    let args = Cli::parse();

    let mut builder = Builder::new();

    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {}",
                buf.default_styled_level(record.level()),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    match args.command {
        Commands::Generate {
            width,
            height,
            dots,
            output,
            color,
            wallpaper,
        } => {
            let img = make_image(width, height, dots, get_color(color));

            let save_path: String;
            info!("Saving image");
            if let Some(path) = output {
                save_path = path;
            } else {
                save_path = format!("{}x{} - {}.png", width, height, dots);
            }

            img.save(&save_path).unwrap();

            if wallpaper {
                info!("Setting image as wallpaper");
                wallpaper::set_from_path(fs::canonicalize(save_path).unwrap().to_str().unwrap())
                    .unwrap();
            }
        }
    }
}

fn make_image(width: u32, height: u32, dots: u64, color: Rgb<u8>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    info!("Creating a Sierpiński triangle with {dots} points on a {width}x{height} image");
    let positions = [
        [width / 10, height - (height / 10)],
        [width - (width / 10), height - (height / 10)],
        [width / 2, height / 10],
    ];

    info!("Creating image");
    let mut img = RgbImage::new(width, height);
    let mut last = [width / 2, height / 2 - 1];

    info!("Placing corners");
    for [x, y] in positions {
        img.put_pixel(x, y, color);
    }

    info!("Placing dots");
    let mut rng = thread_rng();
    let bar = ProgressBar::new(dots);
    for i in 1..=dots {
        let n = rng.gen_range(0..=2);
        img.put_pixel(last[0], last[1], color);
        last = [
            ((last[0] + positions[n][0]) / 2),
            ((last[1] + positions[n][1]) / 2),
        ];
        if i % 1000 == 0 {
            bar.inc(1000);
        }
    }
    bar.finish();

    img
}

fn get_color(hex: Option<String>) -> Rgb<u8> {
    if let Some(hex_code) = hex {
        if hex_code.is_empty() {
            warn!("No hex color provided, using white.");
            return Rgb([255, 255, 255]);
        }

        // Remove # from hex code
        let mut hex_code = if hex_code.starts_with('#') {
            (&hex_code[1..]).to_string()
        } else {
            hex_code
        };

        if !(hex_code.len() == 3 || hex_code.len() == 6) {
            warn!("The length of the provided hex code should be equal to 3 or 6.");
            return Rgb([255, 255, 255]);
        }

        // Convert shorthand hex code to normal hex code (https://en.wikipedia.org/wiki/Web_colors#Shorthand_hexadecimal_form)
        if hex_code.len() == 3 {
            let mut tmp = String::new();
            for c in hex_code.chars() {
                for _ in 0..2 {
                    tmp.push(c);
                }
            }

            hex_code = tmp;
        }

        match (0..hex_code.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&hex_code[i..i + 2], 16))
            .collect::<Result<Vec<u8>, ParseIntError>>()
        {
            Ok(vec) => {
                return Rgb([vec[0], vec[1], vec[2]]);
            }
            Err(error) => {
                match error.kind() {
                    IntErrorKind::InvalidDigit => {
                        warn!("There was an illegal character in the color code, using white.")
                    }
                    _ => warn!("An unknown error occurred while parsing the color, using white."),
                }

                return Rgb([255, 255, 255]);
            }
        }
    } else {
        info!("No hex color provided, using white.");
        return Rgb([255, 255, 255]);
    }
}
