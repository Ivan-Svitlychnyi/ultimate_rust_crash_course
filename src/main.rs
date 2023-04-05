// FINAL PROJECT
//
// Create an image processing application.  Exactly what it does and how it does
// it is up to you, though I've stubbed a good amount of suggestions for you.
// Look for comments labeled **OPTION** below.
//
// Two image files are included in the project root for your convenience: dyson.png and pens.png
// Feel free to use them or provide (or generate) your own images.
//
// Don't forget to have fun and play around with the code!
//
// Documentation for the image library is here: https://docs.rs/image/0.21.0/image/
//
// NOTE 1: Image processing is very CPU-intensive.  Your program will run *noticeably* faster if you
// run it with the `--release` flag.
//
//     cargo run --release [ARG1 [ARG2]]
//
// For example:
//
//     cargo run --release blur image.png blurred.png
//
// NOTE 2: This is how you parse a number from a string (or crash with a
// message). It works with any integer or float type.
//
//     let positive_number: u32 = some_string.parse().expect("Failed to parse a number");

use image::imageops::FilterType::Nearest;
//use std::{thread::sleep, time::Duration};
//use std::{io::Cursor, intrinsics::size_of};
//use image::io::Reader as ImageReader;

fn main() {
    // 1. First, you need to implement some basic command-line argument handling
    // so you can make your program do different things.  Here's a little bit
    // to get you started doing manual parsing.
    //
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }
    let subcommand = args.remove(0);
    match subcommand.as_str() {
        // EXAMPLE FOR CONVERSION OPERATIONS
        "blur" => {
            check_args(&args, 3);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            blur(&args);
        }
        // **OPTION**
        // Brighten -- see the brighten() function below
        "brightness" => {
            check_args(&args, 3);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            brightness(&args);
        }

        // **OPTION**
        // Crop -- see the crop() function below
        "resize" => {
            check_args(&args, 3);
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            resize(&args);
        }

        // **OPTION**
        // Rotate -- see the rotate() function below
        "rotate" => {
            check_args(&args, 3);

            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            rotate(&args);
        }

        // **OPTION**
        // Invert -- see the invert() function below
        "invert" => {
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            invert(&args);
        }
        // **OPTION**
        // Grayscale -- see the grayscale() function below
        "grayscale" => {
            // **OPTION**
            // Improve the blur implementation -- see the blur() function below
            grayscale(&args);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            check_args(&args, 1);
            fractal(&args);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!
        "generate" => {
            check_args(&args, 2);

            generate(&args);
        }

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn check_args(args: &Vec<String>, number_of_arguments: usize) {
    println!("args {:?}", args);
    if args.len() != number_of_arguments {
        print_usage_and_exit();
    }
}

fn print_usage_and_exit() {
    println!(
        "In order to perform operations with the image,
     follow the steps:
    - blur, brightness, resize, rotate(90, 180, 270) - enter three arguments:
      the name of the input file, the name of the output file and
      the number that displays the parameter to be changed;
    - invert, grayscale - two arguments: input file name, output file name;
    - generate -  two arguments: input file name, and color(red, grean, blue);
    - fractal- one argument: the name of the output file."
    );

    std::process::exit(-1);
}

fn blur(args: &[String]) {

    const INFILE: usize = 0;
    const OUTFILE: usize = 1;
    const PARAM: usize = 2;

    // Here's how you open an existing image file
    let img = image::open(&args[INFILE]).expect("Failed to open INFILE.");
    // **OPTION**
    // Parse the blur amount (an f32) from the command-line and pass it through
    // to this function, instead of hard-coding it to 2.0.
    let img2 = img.blur(args[PARAM].parse::<f32>().unwrap());
    // Here's how you save an image to a file.
    img2.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");
}

fn brightness(args: &[String]) {

    const INFILE: usize = 0;
    const OUTFILE: usize = 1;
    const PARAM: usize = 2;
    // See blur() for an example of how to open / save an image.
    let img = image::open(&args[INFILE]).expect("Failed to open INFILE.");

    let img2 = img.brighten(args[PARAM].parse::<i32>().unwrap());

    img2.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");
    // .brighten() takes one argument, an i32.  Positive numbers brighten the
    // image. Negative numbers darken it.  It returns a new image.

    // Challenge: parse the brightness amount from the command-line and pass it
    // through to this function.
}

fn resize(args: &[String]) {
    // See blur() for an example of how to open an image.
    const INFILE: usize = 0;
    const OUTFILE: usize = 1;
    const PARAM: usize = 2;

    let img = image::open(&args[INFILE]).expect("Failed to open INFILE.");

    let img_width = img.width() as f32 * (args[PARAM].parse::<f32>().unwrap() / 100.0);
    let img_height = img.height() as f32 * (args[PARAM].parse::<f32>().unwrap() / 100.0);

    let img2 = img.resize(img_width as u32, img_height as u32, Nearest);

    img2.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    // You may hard-code them, if you like.  It returns a new image.

    // Challenge: parse the four values from the command-line and pass them
    // through to this function.

    // See blur() for an example of how to save the image.
}

fn rotate(args: &[String]) {
    const INFILE: usize = 0;
    const OUTFILE: usize = 1;
    const PARAM: usize = 2;

    // See blur() for an example of how to open an image.
    let img = image::open(&args[INFILE]).expect("Failed to open INFILE.");

    let degrees = args[PARAM].parse::<f32>().unwrap();

    let img2;
    if degrees == 90.0 {
        img2 = img.rotate90();
    } else if degrees == 180.0 {
        img2 = img.rotate180();
    } else if degrees == 270.0 {
        img2 = img.rotate270();
    } else {
        img2 = img;
        print_usage_and_exit();
    }

    img2.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");
    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    // All three methods return a new image.  Pick one and use it!

    // Challenge: parse the rotation amount from the command-line, pass it
    // through to this function to select which method to call.

    // See blur() for an example of how to save the image.
}

fn invert(args: &[String]) {
    // See blur() for an example of how to open an image.
    const INFILE: usize = 0;
    const OUTFILE: usize = 1;

    let img = image::open(&args[INFILE]).expect("Failed to open INFILE.");

    img.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");

    let mut img2 = image::open(&args[OUTFILE]).expect("Failed to open outfile.");
    img2.invert();
    img2.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");

    // .invert() takes no arguments and converts the image in-place, so you
    // will use the same image to save out to a different file.

    // See blur() for an example of how to save the image.
}

fn grayscale(args: &[String]) {
    const INFILE: usize = 0;
    const OUTFILE: usize = 1;

    // See blur() for an example of how to open an image.
    let img = image::open(&args[INFILE]).expect("Failed to open INFILE.");
    let img2 = img.grayscale();

    img2.save(&args[OUTFILE]).expect("Failed writing OUTFILE.");
    // takes no arguments. It returns a new image.

    // See blur() for an example of how to save the image.
}

fn generate(args: &[String]) {
    // Create an ImageBuffer -- see fractal() for an example

    const OUTFILE: usize = 0;
    const PARAM: usize = 1;

    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);
    // Iterate over the coordinates and pixels of the image -- see fractal() for an example
    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    let mut colors: [u8; 3] = [0, 0, 0];
    let color1; //red
    let color2; //grean
    let color3; //blue

    match args[PARAM].as_str() {
        "gren" => {
            color1 = 0;
            color2 = 2;
            color3 = 1;
        }
        "red" => {
            color1 = 2;
            color2 = 0;
            color3 = 1;
        }
        "blue" => {
            color1 = 0;
            color2 = 1;
            color3 = 2;
        }
        _ => {
            color1 = 1;
            color2 = 2;
            color3 = 0;
        }
    }
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        colors[0] = (0.3 * x as f32) as u8; //red
        colors[1] = (0.3 * y as f32) as u8; //blue

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.38, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        //let mut green = 0;//color1
        colors[2] = 0;
        while colors[2] < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            colors[2] += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([colors[color1], colors[color2], colors[color3]]);
    }

    imgbuf.save(&args[OUTFILE]).unwrap();
    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(args: &[String]) {
    const OUTFILE: usize = 0;

    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 3.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(&args[OUTFILE]).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!
