use std::clone::Clone;
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
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 1000;
    const RADIUS: f64 = 400.0;
    const HOLE_RADIUS: f64 = 150.0;
    const BACKGROUND_COLOR: RGBColor = WHITE;
    const BASE_COLOR: RGBColor = RGBColor(99, 153, 61);
    const LABELS_STYLE: (&'static str, i32, &'static RGBColor) = ("monospace", 25, &BLACK);
    const PERCENTAGE_STYLE: (&'static str, i32, &'static RGBColor) = ("monospace", 40, &WHITE);


    pub fn create(parts: Vec<PiePiece>, title_text: &str, center_text: Option<String>) -> InputFile {
        assert!(parts.len() > 0);
        let sizes = parts.iter().map(|x| x.size).collect::<Vec<f64>>();
        let labels = parts.iter().map(|x| x.label.clone()).collect::<Vec<String>>();
        assert_eq!(sizes.len(), labels.len());

        let bytes = Self::create_bytes(sizes, labels, title_text, center_text);
        let mut bytes_png = Vec::new();
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(Self::WIDTH, Self::HEIGHT, bytes).unwrap();

        img.write_to(&mut Cursor::new(&mut bytes_png), ImageFormat::Png).unwrap();
        InputFile::memory(<Vec<u8> as TryInto<bytes::Bytes>>::try_into(bytes_png).unwrap())
    }
    fn create_bytes(sizes: Vec<f64>, labels: Vec<String>, title_text: &str, center_text: Option<String>) -> Vec<u8> {
        let mut bytes = vec![0; Self::WIDTH as usize * Self::HEIGHT as usize * 3];
        {
            let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(bytes.as_mut_slice(), (Self::WIDTH, Self::HEIGHT)).unwrap().into_drawing_area();
            root.fill(&Self::BACKGROUND_COLOR).unwrap();

            let title_style = TextStyle::from(("monospace", 50, "italic").into_font()).color(&(BLACK));
            root.titled(title_text, title_style).unwrap();

            let pie_colors = Self::generate_colors(sizes.len() as u8);
            let dims = root.dim_in_pixel();
            let center = (dims.0 as i32 / 2, dims.1 as i32 / 2);
            let mut pie = Pie::new(&center, &Self::RADIUS, &sizes, &pie_colors, &labels);

            if let Some(text) = center_text {
                let center_text_style = TextStyle::from(("monospace", 40, "bold").into_font()).color(&(BLACK));
                let text_pos = (center.0 - ((text.len() * 9) as i32), center.1 - 10);
                root.draw_text(&text, &center_text_style.into_text_style(&root), text_pos).unwrap();
                pie.donut_hole(Self::HOLE_RADIUS);
            } else {
                pie.donut_hole(Self::HOLE_RADIUS / 2.0);
            }

            pie.start_angle(-180.0);  // отображать по часовой начиная с 9.00
            pie.label_style(Self::LABELS_STYLE.into_text_style(&root));
            pie.percentages(Self::PERCENTAGE_STYLE.into_text_style(&root));
            root.draw(&pie).unwrap();

            root.present().unwrap();
        }
        bytes
    }

    fn generate_colors(size: u8) -> Vec<RGBColor> {
        let mut colors: Vec<RGBColor> = Vec::new();
        colors.push(Self::BASE_COLOR);

        let step = (255 / size).max(40);
        for i in 1..size {
            let next_color = RGBColor(
                Self::BASE_COLOR.0.overflowing_add(i.overflowing_mul(step).0).0,
                Self::BASE_COLOR.1.overflowing_sub(i.overflowing_mul(step).0).0,
                Self::BASE_COLOR.2.overflowing_add(i.overflowing_mul(step.overflowing_mul(2).0).0).0,
            );
            colors.push(next_color);
        }

        colors
    }
}
