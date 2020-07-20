use budget_rs::currency::Currency;
use budget_rs::date::DateInstant;
use budget_rs::location::Location;
use budget_rs::transaction::Category;
use budget_rs::transaction::Transaction;

fn main() {
    let mut t = Transaction::default();
    t.date(DateInstant::from_date(2020, 7, 19))
        .vendor("Chipotle")
        .description("For mom")
        .location(
            Location::default()
                .city("Naperville")
                .state("IL")
                .country("USA")
                .clone(),
        )
        .amount(Currency::new(-1560))
        .category(Category::Restaurants);
    println!("{}", t);
}
