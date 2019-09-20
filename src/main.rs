use raster;
use std::io::{BufRead, Write};
use structopt::StructOpt;

mod scanner;

#[derive(StructOpt)]
#[structopt(name = "scanner", about = "Convert colored text photos to scan-like images.")]
struct Options {
    #[structopt(short, long, default_value = "120")]
    threshold: u8,

    #[structopt(short = "d", long)]
    block_discard_disabled: bool,

    #[structopt(short, long, default_value = "50")]
    block_size: u8,

    #[structopt(short = "f", long, default_value = "80")]
    block_fill_percent: u8,

    input: std::path::PathBuf,
    output: std::path::PathBuf,
}

fn handle_file(input: &std::path::Path, output: &std::path::Path, options: &Options) {
    let mut image = raster::open(input.to_str().unwrap()).unwrap();
    raster::filter::grayscale(&mut image).unwrap();
    scanner::threshhold(&mut image, options.threshold);
    scanner::discard_blocks(&mut image, options.block_size as i32, options.block_fill_percent);
    raster::save(&image, output.to_str().unwrap()).unwrap();
}

fn main() -> std::io::Result<()> {
    let options = Options::from_args();
    if !options.input.exists() {
        println!("Input {:?} does not exists", options.input);
        std::process::exit(1);
    }

    if options.input.is_dir() {
        if options.output.exists() && options.output.is_file() {
            println!("Output exists and is a file, please provide a non existing path or an existing directory.");
            std::process::exit(1);
        } else if !options.output.exists() {
            std::fs::create_dir_all(options.output.clone())?;
        }
    } else if options.input.is_file() {
        if options.output.exists() {
            std::io::stdout()
                .write(b"Output file already exists, overwrite it [y/N]?")
                .unwrap();
            std::io::stdout().flush().unwrap();
            let mut answer = String::new();
            std::io::stdin().lock().read_line(&mut answer)?;
            if answer.trim() != "y" {
                std::process::exit(0);
            }
        }
    }

    if options.input.is_file() {
        handle_file(&options.input, &options.output, &options);
    //        let mut image = raster::open(options.input.to_str().unwrap()).unwrap();
    //        raster::filter::grayscale(&mut image).unwrap();
    //        scanner::threshhold(&mut image, options.threshold);
    //        scanner::discard_blocks(&mut image, options.block_size as i32, options.block_fill_percent);
    //        raster::save(&image, options.output.to_str().unwrap()).unwrap();
    } else {
        let files = std::fs::canonicalize(&options.input)?
            .read_dir()?
            .map(|e| e.unwrap().path())
            .collect::<Vec<_>>();
        for file in &files {
            handle_file(file, &options.output.join(file.file_name().unwrap()), &options);
            //            let mut image = raster::open(file.to_str().unwrap()).unwrap();
            //            raster::filter::grayscale(&mut image).unwrap();
            //            scanner::threshhold(&mut image, options.threshold);
            //            scanner::discard_blocks(&mut image, options.block_size as i32, options.block_fill_percent);
            //            raster::save(&image).unwrap();
        }
    }
    Ok(())
}
