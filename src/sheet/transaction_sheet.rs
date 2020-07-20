use crate::transaction::Transaction;
use crate::currency::Currency;

use std::fmt;

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

    pub fn calc_next(prev: Option<Self>, t: &Transaction) -> Self {
        Self::new(prev.map_or(Currency::default(), |p| p.amount) + *t.get_amount())
    }
}
