use std::env;
use std::process;
use jpeg_converter::Args;
use jpeg_converter::BmpImage;

fn main() {
    let files = Args::get(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let rgb_vec = BmpImage::get_from_file(files.in_file_path);
}
