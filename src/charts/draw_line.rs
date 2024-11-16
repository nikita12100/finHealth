use teloxide::types::InputFile;
use crate::charts::line_series::{Line, LineChart, Series};
use crate::db::portfolio::Portfolio;

pub trait DrawLine {
    fn draw_line_test(&self) -> InputFile;
}

impl DrawLine for Portfolio {
    fn draw_line_test(&self) -> InputFile {
        let data = self.get_all_accounts().iter().map(|account| {
            let series = account.get_balances().iter().map(|balance| {
                Series::new(balance.get_date(), balance.get_amount_bc(&self, account.get_currency()))
            }).collect::<Vec<_>>();
            Line::new(account.get_name(), series)
        }).collect::<Vec<Line>>();


        LineChart::create("История по всем счетам", data)
    }
}