use clap::Parser;

#[derive(Parser)]
struct Args {
    input: String,
    output: String,
}

fn main() {
    let args = Args::parse();

    let img = image::open(&args.input).expect("failed to open image");
    let rgb = img.to_rgb8();

    rgb.save_with_format(&args.output, image::ImageFormat::Jpeg).expect("failed to save image");
}
