use image::{Rgb, RgbImage};
use indicatif::ProgressBar;
use rand::{thread_rng, Rng};

fn main() {
    make_image(1920, 1080, 100000000);
}

fn make_image(width: u32, height: u32, dots: u64) {
    println!("Creating a Sierpiński triangle with {dots} points on a {width}x{height} image");
    let positions = [
        [width / 10, height - (height / 10)],
        [width - (width / 10), height - (height / 10)],
        [width / 2, height / 10],
    ];

    println!("Creating image");
    let mut img = RgbImage::new(width, height);
    let mut last = [width / 2, height / 2 - 1];

    println!("Placing corners");
    for [x, y] in positions {
        img.put_pixel(x, y, Rgb([255, 255, 255]));
    }

    println!("Placing dots");
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

    println!("Saving image");
    img.save(format!("{width}x{height} - {dots}.png")).unwrap();
}
