use std::io::Cursor;
use plotters::prelude::*;
use chrono::{Utc, TimeZone, DateTime, Date};
use image::{ImageBuffer, ImageFormat, Rgb};
use plotters::backend::RGBPixel;
use teloxide::types::InputFile;

pub struct Series {
    pub time: DateTime<Utc>,
    pub value: u32,
}

impl Series {
    pub fn new(time: DateTime<Utc>, value: u32) -> Series { Series { time, value } }
}

pub struct Line {
    pub label: String,
    pub series: Vec<Series>,
}

impl Line {
    pub fn get_start_date(&self) -> DateTime<Utc> {
        self.series.iter().map(|x| x.time).min().unwrap()
    }
    pub fn get_end_date(&self) -> DateTime<Utc> {
        self.series.iter().map(|x| x.time).max().unwrap()
    }
    pub fn get_max_value(&self) -> u32 {
        self.series.iter().map(|s| s.value).max().unwrap()
    }
    pub fn get_min_value(&self) -> u32 {
        self.series.iter().map(|s| s.value).min().unwrap()
    }
}

pub struct LineChart {}

impl LineChart {
    const WIDTH: u32 = 1200;
    const HEIGHT: u32 = 1000;
    const BACKGROUND_COLOR: RGBColor = WHITE;

    pub fn create(title_text: &str, data_raw: Line) -> InputFile {
        // assert!(data_raw.len() > 0);
        // let x_left = data_raw.iter().map(|d| d.get_start_date()).min().unwrap();
        // let x_right = data_raw.iter().map(|d| d.get_end_date()).max().unwrap();
        // let y_bottom = data_raw.iter().map(|d| d.get_min_value()).min().unwrap();
        // let y_top = data_raw.iter().map(|d| d.get_max_value()).max().unwrap();
        // let data = data_raw.iter().flat_map(|d| d.series).collect::<Vec<Series>>();
        let x_left = data_raw.get_start_date();
        let x_right = data_raw.get_end_date();
        let y_bottom = data_raw.get_min_value();
        let y_top = data_raw.get_max_value();
        let data = data_raw.series.iter().map(|s| (s.time, s.value)).collect::<Vec<_>>();

        let bytes = Self::create_bytes(title_text, x_left, x_right, y_bottom, y_top, data);
        let mut bytes_png = Vec::new();
        let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(Self::WIDTH, Self::HEIGHT, bytes).unwrap();

        img.write_to(&mut Cursor::new(&mut bytes_png), ImageFormat::Png).unwrap();
        InputFile::memory(<Vec<u8> as TryInto<bytes::Bytes>>::try_into(bytes_png).unwrap())
    }

    fn create_bytes(
        title_text: &str,
        x_left: DateTime<Utc>,
        x_right: DateTime<Utc>,
        y_bottom: u32,
        y_top: u32,
        data: Vec<(DateTime<Utc>, u32)>,
    ) -> Vec<u8> {
        let mut bytes = vec![0; Self::WIDTH as usize * Self::HEIGHT as usize * 3];
        {
            let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(bytes.as_mut_slice(), (Self::WIDTH, Self::HEIGHT)).unwrap().into_drawing_area();
            root.fill(&Self::BACKGROUND_COLOR).unwrap();

            let mut ctx = ChartBuilder::on(&root)
                .set_label_area_size(LabelAreaPosition::Left, 50)
                .set_label_area_size(LabelAreaPosition::Right, 50)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(title_text, ("sans-serif", 40))
                .build_cartesian_2d(x_left..x_right, y_bottom..y_top)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();
            ctx.draw_series(LineSeries::new(data, &BLUE)).unwrap();
        }
        bytes
    }
}
