// use plotters::prelude::*;
// use chrono::{Utc, TimeZone};
//
// const DATA: [f64; 14] = [137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.10, 138.24, 135.67, 137.12, 138.12];
//
// let root_area = BitMapBackend::new("draw.png", (600, 400))
// .into_drawing_area();
// root_area.fill(&CYAN).unwrap();
//
// let start_date = Utc.ymd(2019, 10, 1);
// let end_date = Utc.ymd(2019, 10, 18);
//
// let mut ctx = ChartBuilder::on(&root_area)
// .set_label_area_size(LabelAreaPosition::Left, 40)
// .set_label_area_size(LabelAreaPosition::Bottom, 40)
// .caption("test draw", ("sans-serif", 40))
// .build_cartesian_2d(start_date..end_date, 130.0..145.0)
// .unwrap();
//
// ctx.configure_mesh().draw().unwrap();
//
// ctx.draw_series(
// LineSeries::new(
// (0..).zip(DATA.iter()).map(|(idx, price)| {
// let day = (idx / 5) * 7 + idx % 5 + 1;
// let date = Utc.ymd(2019, 10, day);
// (date, *price)
// }),
// &BLUE,
// )
// ).unwrap();