use image::Luma;
use image::{GenericImageView, ImageBuffer, ImageError, Pixel};
use ndarray::{arr2, Array};

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
    // let input_path =
        // "/home/jono/Projects/Accelerant/codenoir/codenoir/content/posts/gallery/Lenna.png";
    // let input_path = "/home/jono/Projects/Accelerant/codenoir/codenoir/content/posts/gallery/bigcombo.jpg";
    let input_path = "/home/jono/Projects/Accelerant/codenoir/codenoir/content/posts/gallery/ny.jpg";
    let output_path = "/home/jono/Projects/Accelerant/desat/tmp.png";
    let output_gray_path = "/home/jono/Projects/Accelerant/desat/tmp_gray.png";
    match desat(input_path, output_path) {
        Ok(_) => println!("Completed Noir Conversion!"),
        Err(e) => println!("Error: {}", e),
    }
    match gray(input_path, output_gray_path) {
        Ok(_) => println!("Completed Gray Conversion!"),
        Err(e) => println!("Error: {}", e),
    }
}
