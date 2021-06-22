#![allow(non_snake_case)]
extern crate image;
use std::io;
use std::io::*;

pub mod Identicon {
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
            
            let mut i: usize = 0;
            let mut m: usize = 0;
            while i < hash.len() {
                

                //Edge
                if (hash[i] % 2) == 0 {
                    grid[m] = true;
                } else {
                    grid[m] = false;
                }
                

                //Aisle
                if (hash[i+1] % 2) == 0 {
                    grid[m+1] = true;
                } else {
                    grid[m+1] = false;
                }
                

                //Centre
                if (hash[i+2] % 2) == 0 {
                    grid[m+2] = true;
                } else {
                    grid[m+2] = false;
                }

                //Mirror
                grid[m+4] = grid[m];
                grid[m+3] = grid[m+1];

                //If i equals 12 and the repeat counter equals 0 then exit the loop.
                if i == 12 {
                    break;
                }

                //Increment counters
                m += 5;
                i += 3;
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
            resized.save(format!("{}.png", fileName)).unwrap();
        }
    }
}
