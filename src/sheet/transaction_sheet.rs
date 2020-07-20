use crate::currency::Currency;
use crate::transaction::Transaction;
use crate::sheet::Sheet;

use std::fmt;

pub struct TransactionSheet {
    title: String,
    rows: Vec<(Transaction, Balance)>,
}

impl TransactionSheet {
    pub fn update_balances(prev_row: Option<(Transaction, Balance)>, rows: &mut [(Transaction, Balance)]) {
        let mut prev = prev_row.map_or(Balance::default(), |r| r.1.clone());
        for r in rows.iter_mut() {
            r.1 = Balance::calc_next(&prev, &r.0);
            prev = r.1.clone();
        }
    }
}

impl Sheet for TransactionSheet {
    type Row = (Transaction, Balance);
    type RowFind = u32;
    type RowHandle = Self::Row;

    fn title(&self) -> &str {
        self.title.as_str()
    }
    fn headers(&self) -> Vec<&str> {
        vec!["date", "vendor", "description", "location", "amount", "category", "balance"]
    }
    fn lines(&self) -> Vec<String> {
        let mut v = Vec::with_capacity(self.rows.len());
        for r in &self.rows {
            v.push(format!("{},{}", r.0, r.1));
        }
        v
    }
    fn insert(&mut self, row: Self::Row) {
        self.rows.push(row);
        self.rows.sort();
        TransactionSheet::update_balances(None, &mut self.rows);
    }
    fn update(&mut self, old_row: &Self::RowFind, new_row: Self::Row) {
        self.rows.push(new_row);
        self.rows.swap_remove(*old_row as usize);
        self.rows.sort();
        TransactionSheet::update_balances(None, &mut self.rows);
    }
    fn get_row(&self, find: &Self::RowFind) -> Option<Self::RowHandle> {
        self.rows.get(*find as usize).cloned()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Balance {
    amount: Currency,
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.amount.to_string_numeric())
    }
}

impl Balance {
    pub fn new(amount: Currency) -> Self {
        Self { amount }
    }

    pub fn calc_next(prev: &Self, t: &Transaction) -> Self {
        Self::new(prev.amount + *t.get_amount())
    }
}
