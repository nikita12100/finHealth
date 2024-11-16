use plotters::style::RGBColor;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn total_sum_spaced(total_summ: u32) -> String {
    let mut total_sum_str: Vec<char> = Vec::new();
    for (i, char) in total_summ.to_string().chars().rev().enumerate() {
        if i % 3 == 0 {
            total_sum_str.push(' ');
            total_sum_str.push(char);
        } else {
            total_sum_str.push(char);
        }
    }
    total_sum_str.reverse();
    total_sum_str.iter().collect()
}

pub fn generate_colors(size: u8, base_color: RGBColor) -> Vec<RGBColor> {
    let mut colors: Vec<RGBColor> = Vec::new();
    colors.push(base_color);

    let step = (255 / size).max(40);
    for i in 1..size {
        let next_color = RGBColor(
            base_color.0.overflowing_add(i.overflowing_mul(step).0).0,
            base_color.1.overflowing_sub(i.overflowing_mul(step).0).0,
            base_color.2.overflowing_add(i.overflowing_mul(step.overflowing_mul(2).0).0).0,
        );
        colors.push(next_color);
    }

    colors
}

pub fn make_keyboard(row_size: usize, buttons: Vec<&str>) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<Vec<InlineKeyboardButton>> = vec![];

    for versions in buttons.chunks(row_size) {
        let row = versions
            .iter()
            .map(|&version| InlineKeyboardButton::callback(version.to_owned(), version.to_owned()))
            .collect();

        keyboard.push(row);
    }

    InlineKeyboardMarkup::new(keyboard)
}
