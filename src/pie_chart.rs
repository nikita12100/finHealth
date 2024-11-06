use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::*;
use std::io::Cursor;
use image::{ImageBuffer, ImageFormat, Rgb};
use teloxide::types::InputFile;


pub struct PieChart {}
impl PieChart {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 600;
    pub fn create_file() -> InputFile {
        let bytes = Self::create_bytes();
        let mut bytes_png = Vec::new();
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(Self::WIDTH, Self::HEIGHT, bytes).unwrap();

        img.write_to(&mut Cursor::new(&mut bytes_png), ImageFormat::Png).unwrap();
        InputFile::memory(<Vec<u8> as TryInto<bytes::Bytes>>::try_into(bytes_png).unwrap())
    }
    fn create_bytes() -> Vec<u8> {
        let mut bytes = vec![0; Self::WIDTH as usize * Self::HEIGHT as usize * 3];
        {
            let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(bytes.as_mut_slice(), (Self::WIDTH, Self::HEIGHT)).unwrap().into_drawing_area();
            root.fill(&WHITE).unwrap();
            root.draw(&Pie::new(
                &(300, 300), &250.0, &[50.0, 25.25, 20.0, 5.5], &[RED, BLUE, GREEN, WHITE], &["Red", "Blue", "Green", "White"])
            ).unwrap();
            root.present().unwrap();
        }
        bytes
    }
}