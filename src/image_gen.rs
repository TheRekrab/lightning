use image::{ImageBuffer, Rgb};

use crate::{
    constants::{bolts, HEIGHT, WIDTH},
    lightning_path::LightningPath,
    maze::Maze,
};

pub fn generate_image() {
    let mut image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    for bolt in bolts() {
        let rgb = Rgb(bolt.color);
        let lightning = Maze::new();
        let res: Option<LightningPath> = lightning.solve();
        if let Some(path) = res {
            for coord in path.path {
                image.put_pixel(coord.x as u32, coord.y as u32, rgb);
            }
        }
    }

    image
        .save("lightning.jpg")
        .expect("Error while writing to image file");
}
