use image::Luma;
use image::{GenericImageView, ImageBuffer, ImageError, Pixel};
use ndarray::{arr2, Array};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "desat")]
#[command(about = "Desaturate an image", long_about = None)]
struct Args {
    /// input image file path/name
    #[arg(short, required = true)]
    input_file: PathBuf,
    /// output grayscale file path/name
    #[arg(short('g'), long("gray"), default_value = "gray.png")]
    ouptut_gray_file: PathBuf,
    /// output noir file path/name
    #[arg(short('n'), long("noir"), default_value = "noir.png")]
    output_noir_file: PathBuf,
}

fn clamp(val: f64) -> u8 {
    if val < 0.0 {
        0
    } else if val > 255.0 {
        255
    } else {
        val.round() as u8
    }
}

fn gray(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path).expect("File not found!");
    img.grayscale().save(output_path)?;
    Ok(())
}

//https://blog.logrocket.com/decoding-encoding-images-rust-using-image-crate/
fn desat(input_path: &str, output_path: &str) -> Result<(), ImageError> {
    let img = image::open(input_path).expect("File not found!");
    let (w, h) = img.dimensions();
    // create a new buffer for our output
    let mut output = ImageBuffer::new(w, h);

    for (x, y, pixel) in img.pixels() {
        let channels = Pixel::channels(&pixel);
        // YUV
        let rgb = Array::from_shape_vec(
            (3, 1),
            vec![
                f64::from(channels[0]),
                f64::from(channels[1]),
                f64::from(channels[2]),
            ],
        )
        .unwrap();
        let conversion_matrix = arr2(&[
            // [0.299000, 0.587000, 0.114000], // https://en.wikipedia.org/wiki/YUV#Conversion_to/from_RGB
            [0.2126, 0.7152, 0.0722], // https://www.gimp.org/tutorials/Digital_Black_and_White_Conversion/
            [-0.14713, -0.28886, 0.436],
            [0.615, -0.51499, -0.10001],
        ]);
        // https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/linear_algebra.html#multiplying-matrices
        let yuv = (conversion_matrix.dot(&rgb)).into_raw_vec();
        let new_pixel = Luma([clamp(yuv[0])]);
        output.put_pixel(x, y, new_pixel);
    }

    // let output_gray= DynamicImage::from(output).grayscale();
    output.save(output_path)?;
    Ok(())
}

fn main() {

    let args = Args::parse();

    let input_path = args.input_file.to_str().expect("reading input file path");
    println!("Converting input_file: {}", &input_path);

    let output_path = args.output_noir_file.to_str().expect("reading output gray file path");
    let output_gray_path = args.ouptut_gray_file.to_str().expect("reading output noir file path");
    match desat(input_path, output_path) {
        Ok(_) => println!("Completed Noir Conversion!"),
        Err(e) => println!("Error: {}", e),
    }
    match gray(input_path, output_gray_path) {
        Ok(_) => println!("Completed Gray Conversion!"),
        Err(e) => println!("Error: {}", e),
    }
}
