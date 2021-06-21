extern crate image;
//extern crate num_complex;

use image::{imageops};

mod Identicon {
    pub struct Identicon {
        pub input: String,
        pub hash: [u8; 16],
        pub rgb: [u8; 3],
        pub grid: [bool; 25]
    }

    impl Identicon {
        pub fn new(input: String) -> Identicon {
            let input_clone: String = input.clone();  //Clone input to own input here and be able to copy it.
            let hash: [u8; 16] = Identicon::crypt(input);
            let rgb: [u8; 3] = Identicon::extractIntsFromHash(&hash);
            let grid: [bool; 25] = Identicon::createGrid(&hash);

            //Display grid for debugging purposes.
            // println!("Grid:");
            // println!("{},{},{},{},{}", grid[0], grid[1], grid[2], grid[3], grid[4]);
            // println!("{},{},{},{},{}", grid[5], grid[6], grid[7], grid[8], grid[9]);
            // println!("{},{},{},{},{}", grid[10], grid[11], grid[12], grid[13], grid[14]);
            // println!("{},{},{},{},{}", grid[15], grid[16], grid[17], grid[18], grid[19]);
            // println!("{},{},{},{},{}", grid[20], grid[21], grid[22], grid[23], grid[24]);
            
            return Identicon {
                input: input_clone,
                hash: hash,
                rgb: rgb,
                grid: grid
            }
        }

        fn crypt(input: String) -> [u8; 16] {
            //Hash input.
            let hash: md5::Digest = md5::compute(input);
            let mut retArray: [u8; 16] = [0; 16];

            //Extract md5::Digest input into a [u8; 16] array.
            let mut i: usize = 0;
            while i < 16 {
                retArray[i] = hash[i];
                i = i + 1;
            }

            retArray
        }

        fn extractIntsFromHash(hash: &[u8; 16]) -> [u8; 3] {
            //Extract first 3 values from md5 digest and use as RGB colour.
            [hash[0], hash[1], hash[2]]
        }

        fn createGrid(hash: &[u8; 16]) -> [bool; 25] {
            let mut grid: [bool; 25] = [true; 25];
            
            //Impl algorithm to create grid here.

            let mut i: usize = 0;
            while i < 25 {
                if i % 2 == 0 {
                    grid[i] = true;
                } else {
                    grid[i] = false;
                }
                i += 1;
            }

            grid
        }

        pub fn render(&self, fileName: &str) {
            //Define image dimensions
            let imgx = 5;
            let imgy = 5;

            // Create a new ImgBuf with width: imgx and height: imgy
            let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

            // Iterate over the coordinates and pixels of the image and set colour.
            let mut i: usize = 0;
            for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
                if self.grid[i] {
                    let r = self.rgb[0];
                    let g = self.rgb[1];
                    let b = self.rgb[2];
                    *pixel = image::Rgb([r, g, b]);
                } else {
                    *pixel = image::Rgb([255, 255, 255]);
                }
                i=i+1;
            }

            //Resize image to be larger
            let resized = image::imageops::resize(&imgbuf, 128, 128, image::imageops::FilterType::Nearest);

            //Save the  resized image
            resized.save(format!("./{}.png", fileName)).unwrap();
        }
    }
}



fn main() {
    let input: String = String::from("nlanson");
    let identicon: Identicon::Identicon = Identicon::Identicon::new(input);
    identicon.render("identicon");
}