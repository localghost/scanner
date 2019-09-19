use raster;
use structopt::StructOpt;

mod scanner;

#[derive(StructOpt)]
#[structopt(name = "scanner", about = "Convert colored text photos to scan-like images.")]
struct Options {
    #[structopt(short, long, default_value = "120")]
    threshold: u8,

    #[structopt(short, long, default_value = "50")]
    block_size: u8,

    #[structopt(short = "f", long, default_value = "80")]
    block_fill_percent: u8,

    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn main() {
    let options = Options::from_args();
    if !options.input.exists() {
        println!("Input {:?} does not exists", options.input);
        std::process::exit(1);
    }

    if options.input.is_dir() {
        if options.output.exists() && options.output.is_file() {
            println!("Output exists and is a file, please provide non existing path or an existing directory.");
            std::process::exit(1);
        }
    }

    let files = if options.input.is_file() {
        vec![options.input]
    } else {
        std::fs::canonicalize(&options.input)
            .unwrap()
            .read_dir()
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect::<Vec<_>>()
    };

    let mut image = raster::open("/home/kostrzewa/Pictures/page.jpg").unwrap();
    raster::filter::grayscale(&mut image).unwrap();
    scanner::threshhold(&mut image, options.threshold);
    scanner::discard_blocks(&mut image, options.block_size as i32, options.block_fill_percent);
    raster::save(&image, "/tmp/page_bw.jpg").unwrap();
}
