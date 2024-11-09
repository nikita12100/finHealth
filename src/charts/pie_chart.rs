use plotters::backend::{BitMapBackend, RGBPixel};
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::*;
use std::io::Cursor;
use image::{ImageBuffer, ImageFormat, Rgb};
use teloxide::types::InputFile;

pub struct PiePiece {
    pub size: f64,
    pub label: String,
}

pub struct PieChart {}
impl PieChart {
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 600;
    const RADIUS: f64 = 250.0;
    const BACKGROUND_COLOR: RGBColor = WHITE;
    const COLORS: [RGBColor; 7] = [RED, BLUE, GREEN, BLACK, YELLOW, CYAN, MAGENTA];

    pub fn create(parts: Vec<PiePiece>) -> InputFile {
        let sizes = parts.iter().map(|x| x.size).collect::<Vec<f64>>();
        let labels = parts.iter().map(|x| x.label.clone()).collect::<Vec<String>>();
        assert_eq!(sizes.len(), labels.len());
        assert!(sizes.len() <= 7);

        let bytes = Self::create_bytes(sizes, labels);
        let mut bytes_png = Vec::new();
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(Self::WIDTH, Self::HEIGHT, bytes).unwrap();

        img.write_to(&mut Cursor::new(&mut bytes_png), ImageFormat::Png).unwrap();
        InputFile::memory(<Vec<u8> as TryInto<bytes::Bytes>>::try_into(bytes_png).unwrap())
    }
    fn create_bytes(sizes: Vec<f64>, labels: Vec<String>) -> Vec<u8> {
        let mut bytes = vec![0; Self::WIDTH as usize * Self::HEIGHT as usize * 3];
        let center = ((Self::WIDTH as f32 / 2.0) as i32, (Self::HEIGHT as f32 / 2.0) as i32);
        {
            let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(bytes.as_mut_slice(), (Self::WIDTH, Self::HEIGHT)).unwrap().into_drawing_area();
            root.fill(&Self::BACKGROUND_COLOR).unwrap();
            root.draw(&Pie::new(&center, &Self::RADIUS, &sizes, &Self::COLORS, &labels)).unwrap();
            root.present().unwrap();
        }
        bytes
    }
}
