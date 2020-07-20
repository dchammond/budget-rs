use crate::currency::Currency;
use crate::date::{DateDuration, DateInstant};
use crate::location::Location;

use std::convert::TryFrom;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Transaction {
    date: DateInstant,
    vendor: String,
    description: String,
    location: Location,
    amount: Currency,
    category: Category,
}

impl Transaction {
    pub fn date(&mut self, date: DateInstant) -> &mut Self {
        self.date = date;
        self
    }
    pub fn get_date(&self) -> &DateInstant {
        &self.date
    }
    pub fn vendor<'a, 'b>(&'a mut self, vendor: &'b str) -> &'a mut Self {
        self.vendor = vendor.to_owned();
        self
    }
    pub fn get_vendor(&self) -> &str {
        self.vendor.as_str()
    }
    pub fn description<'a, 'b>(&'a mut self, description: &'b str) -> &'a mut Self {
        self.description = description.to_owned();
        self
    }
    pub fn get_description(&self) -> &str {
        self.description.as_str()
    }
    pub fn location(&mut self, location: Location) -> &mut Self {
        self.location = location;
        self
    }
    pub fn get_location(&self) -> &Location {
        &self.location
    }
    pub fn amount(&mut self, amount: Currency) -> &mut Self {
        self.amount = amount;
        self
    }
    pub fn get_amount(&self) -> &Currency {
        &self.amount
    }
    pub fn category(&mut self, category: Category) -> &mut Self {
        self.category = category;
        self
    }
    pub fn get_category(&self) -> &Category {
        &self.category
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub(crate) enum Category {
    Auto,
    CreditCard,
    Donation,
    Electronics,
    Entertainment,
    Gas,
    Gift,
    Groceries,
    Investment,
    Medical,
    Pay,
    Personal,
    Rent,
    Restaurants,
    Savings,
    School,
    Shopping,
    Subscriptions,
    Taxes,
    Travel,
    Transfer,
    Utilities,
    Other,
}

impl Default for Category {
    fn default() -> Self {
        Self::Other
    }
}

impl TryFrom<&str> for Category {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ret = match s.to_lowercase().as_str() {
            "auto" => Category::Auto,
            "creditcard" => Category::CreditCard,
            "donation" => Category::Donation,
            "electronics" => Category::Electronics,
            "entertainment" => Category::Entertainment,
            "gas" => Category::Gas,
            "gift" => Category::Gift,
            "groceries" => Category::Groceries,
            "investment" => Category::Investment,
            "medical" => Category::Medical,
            "pay" => Category::Pay,
            "personal" => Category::Personal,
            "rent" => Category::Rent,
            "restaurants" => Category::Restaurants,
            "savings" => Category::Savings,
            "school" => Category::School,
            "shopping" => Category::Shopping,
            "subscriptions" => Category::Subscriptions,
            "taxes" => Category::Taxes,
            "travel" => Category::Travel,
            "transfer" => Category::Transfer,
            "utilities" => Category::Utilities,
            _ => Category::Other,
        };
        match ret {
            Category::Other => Err(format!("No such category: {}", s)),
            _ => Ok(ret),
        }
    }
}

impl ToString for Category {
    fn to_string(&self) -> String {
        match self {
            Self::Auto => "auto",
            Self::CreditCard => "creditcard",
            Self::Donation => "donation",
            Self::Electronics => "electronics",
            Self::Entertainment => "entertainment",
            Self::Gas => "gas",
            Self::Gift => "gift",
            Self::Groceries => "groceries",
            Self::Investment => "investment",
            Self::Medical => "medical",
            Self::Pay => "pay",
            Self::Personal => "personal",
            Self::Rent => "rent",
            Self::Restaurants => "restaurants",
            Self::Savings => "savings",
            Self::School => "school",
            Self::Shopping => "shopping",
            Self::Subscriptions => "subscriptions",
            Self::Taxes => "taxes",
            Self::Travel => "travel",
            Self::Transfer => "transfer",
            Self::Utilities => "utilities",
            Self::Other => "other",
        }
        .to_owned()
    }
}
