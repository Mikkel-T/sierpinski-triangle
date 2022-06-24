use clap::Parser;
use env_logger::Builder;
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use log::{info, LevelFilter};
use rand::{thread_rng, Rng};
use std::io::Write;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Width of the image (In pixels)
    #[clap(short, long)]
    width: u32,

    /// Height of the image (In pixels)
    #[clap(short, long)]
    height: u32,

    /// Number of dots to draw on the image
    #[clap(short, long)]
    dots: u64,
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

    make_image(args.width, args.height, args.dots);
}

fn make_image(width: u32, height: u32, dots: u64) {
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
        img.put_pixel(x, y, Rgb([255, 255, 255]));
    }

    info!("Placing dots");
    let mut rng = thread_rng();
    let bar = ProgressBar::new(dots);
    for i in 1..=dots {
        let n = rng.gen_range(0..=2);
        img.put_pixel(last[0], last[1], Rgb([255, 255, 255]));
        last = [
            ((last[0] + positions[n][0]) / 2),
            ((last[1] + positions[n][1]) / 2),
        ];
        if i % 1000 == 0 {
            bar.inc(1000);
        }
    }
    bar.finish();

    info!("Saving image");
    img.save(format!("{width}x{height} - {dots}.png")).unwrap();
}
