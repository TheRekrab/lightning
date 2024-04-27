use image::{ImageBuffer, Rgb};

use crate::coord::Coord;

pub fn generate_image(width: usize, height: usize, path: &Vec<Coord>) {
    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width as u32, height as u32);

    for coord in path {
        image.put_pixel(coord.x as u32, coord.y as u32, Rgb([255, 255, 255]));
    }

    image
        .save("lightning.jpg")
        .expect("Error while saving to file");
}
