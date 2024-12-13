use crate::charts::pie_chart::{PieChart, PiePiece};
use crate::db::portfolio::Portfolio;
use crate::enums::category::Category;
use crate::utils::common::total_sum_spaced;
use chrono::{Days, Utc};
use itertools::Itertools;
use std::collections::HashMap;
use teloxide::types::InputFile;

pub trait DrawPie {
    fn draw_pie_name_allocations(&self) -> InputFile;
    fn draw_pie_spends(&self, account_name: String, num_days: u32) -> InputFile;
    fn draw_pie_type_allocations(&self) -> InputFile;
    fn draw_pie_location_allocations(&self) -> InputFile;
    fn draw_pie_currency_allocations(&self) -> InputFile;
    fn draw_pie_from_distribution(distribution: HashMap<String, u32>, title: &str) -> InputFile;
}

impl DrawPie for Portfolio {
    fn draw_pie_name_allocations(&self) -> InputFile {
        let mut distribution_amount: HashMap<String, u32> = HashMap::new();
        for account in self.get_all_accounts().iter() {
            distribution_amount
                .entry(account.get_name())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self).unwrap())
                .or_insert(account.get_last_amount_bc(&self).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_amount, "Срез по всем балансам")
    }

    fn draw_pie_spends(&self, account_name: String, num_days: u32) -> InputFile {
        let week_threshold = Utc::now()
            .checked_sub_days(Days::new(num_days as u64))
            .unwrap();
        let account = self
            .get_all_accounts()
            .iter()
            .find(|account| account.get_name() == account_name)
            .unwrap();

        let mut distribution_spends: HashMap<String, u32> = HashMap::new();
        for (balance_prev, balance) in account.get_balances().into_iter().tuple_windows() {
            let spend = balance_prev.get_amount() as i32 - balance.get_amount() as i32;
            if spend > 0 && balance.get_date() > week_threshold {
                distribution_spends
                    .entry(
                        balance
                            .get_category()
                            .map(|c| c.name())
                            .unwrap_or(Category::default().to_string()),
                    )
                    .and_modify(|sum| *sum += spend as u32)
                    .or_insert(spend as u32);
            }
        }

        Self::draw_pie_from_distribution(
            distribution_spends,
            &format!("Траты за {} дней", num_days),
        )
    }

    fn draw_pie_type_allocations(&self) -> InputFile {
        let mut distribution_type: HashMap<String, u32> = HashMap::new();
        for account in self.get_all_accounts().iter() {
            distribution_type
                .entry(account.get_type().to_string())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self).unwrap())
                .or_insert(account.get_last_amount_bc(&self).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_type, "Срез по всем балансам в типах")
    }

    fn draw_pie_location_allocations(&self) -> InputFile {
        let mut distribution_location: HashMap<String, u32> = HashMap::new();
        for account in self.get_all_accounts().iter() {
            distribution_location
                .entry(account.get_location().to_string())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self).unwrap())
                .or_insert(account.get_last_amount_bc(&self).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_location, "Срез по всем балансам в локациях")
    }

    fn draw_pie_currency_allocations(&self) -> InputFile {
        let mut distribution_currency: HashMap<String, u32> = HashMap::new();
        for account in self.get_all_accounts().iter() {
            distribution_currency
                .entry(account.get_currency().to_string())
                .and_modify(|sum| *sum += account.get_last_amount_bc(&self).unwrap())
                .or_insert(account.get_last_amount_bc(&self).unwrap());
        }

        Self::draw_pie_from_distribution(distribution_currency, "Срез по всем балансам в валютах")
    }

    fn draw_pie_from_distribution(distribution: HashMap<String, u32>, title: &str) -> InputFile {
        let mut parts: Vec<PiePiece> = Vec::new();
        let mut total_summ = 0;
        for (key, value) in distribution {
            total_summ += value;
            parts.push(PiePiece {
                size: value as f64,
                label: key,
            });
        }

        PieChart::create(parts, title, Some(total_sum_spaced(total_summ)))
    }
}

#[cfg(test)]
mod tests {
    use crate::charts::draw_pie::DrawPie;
    use crate::utils::mock_data::MockData;

    #[test]
    fn fails_when_draw() {
        let portfolio = MockData::create();
        portfolio.draw_pie_name_allocations();
        assert_eq!(1, 1);
    }
}
