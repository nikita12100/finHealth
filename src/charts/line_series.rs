use std::io::Cursor;
use plotters::prelude::*;
use chrono::{Utc, DateTime};
use image::{ImageBuffer, ImageFormat, Rgb};
use plotters::backend::RGBPixel;
use plotters::coord::types::RangedCoordu32;
use teloxide::types::InputFile;
use crate::charts::pie_chart::PieChart;

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
    pub fn new(label: String, series: Vec<Series>) -> Line { Line{ label, series } }
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
    const LABELS_TEXT_STYLE: (&'static str, i32, &'static RGBColor) = ("sans-serif", 15, &BLACK);
    const TITLE_TEXT_STYLE: (&'static str, i32, &'static RGBColor) = ("monospace", 40, &BLACK);
    const BASE_COLOR: RGBColor = RGBColor(99, 153, 61);


    pub fn create(title_text: &str, data_raw: Vec<Line>) -> InputFile {
        assert!(data_raw.len() > 0);
        let x_left = data_raw.iter().map(|d| d.get_start_date()).min().unwrap();
        let x_right = data_raw.iter().map(|d| d.get_end_date()).max().unwrap();
        let y_bottom = (data_raw.iter().map(|d| d.get_min_value()).min().unwrap() as f32 * 0.999) as u32;
        let y_top = (data_raw.iter().map(|d| d.get_max_value()).max().unwrap() as f32 * 1.01) as u32;

        let bytes = Self::create_bytes(title_text, x_left, x_right, y_bottom, y_top, data_raw);
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
        data_raw: Vec<Line>,
    ) -> Vec<u8> {
        let mut bytes = vec![0; Self::WIDTH as usize * Self::HEIGHT as usize * 3];
        {
            let root = BitMapBackend::<RGBPixel>::with_buffer_and_format(bytes.as_mut_slice(), (Self::WIDTH, Self::HEIGHT)).unwrap().into_drawing_area();
            root.fill(&Self::BACKGROUND_COLOR).unwrap();

            let mut ctx: ChartContext<BitMapBackend, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordu32>> = ChartBuilder::on(&root)
                .margin(20)
                .set_label_area_size(LabelAreaPosition::Left, 50)
                .set_label_area_size(LabelAreaPosition::Right, 50)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .caption(title_text, Self::TITLE_TEXT_STYLE)
                .build_cartesian_2d(x_left..x_right, y_bottom..y_top)
                .unwrap();

            ctx.configure_mesh().draw().unwrap();

            let colors = PieChart::generate_colors(data_raw.len() as u8);
            for (i, line) in data_raw.iter().enumerate() {
                let series = line.series.iter().map(|s| (s.time, s.value)).collect::<Vec<_>>();
                Self::append_series(&mut ctx, series, line.label.clone(), colors[i]);
            }

            root.present().unwrap();
        }
        bytes
    }

    fn append_series<'a>(
        ctx: &mut ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedDateTime<DateTime<Utc>>, RangedCoordu32>>,
        series: Vec<(DateTime<Utc>, u32)>,
        label_text: String,
        line_color: RGBColor,
    ) {
        ctx
            .draw_series(LineSeries::new(series, line_color)).unwrap()
            .label(label_text)
            .legend(move |(x,y)| Rectangle::new([(x + 15, y + 1), (x, y)], line_color));
        ctx
            .configure_series_labels()
            .position(SeriesLabelPosition::LowerRight)
            // .position(SeriesLabelPosition::Coordinate(1, 1))
            // .margin(10)
            // .legend_area_size(5)
            .background_style(&WHITE.mix(0.8))
            .label_font(Self::LABELS_TEXT_STYLE)
            .border_style(&BLACK)
            .draw().unwrap();
    }
}
