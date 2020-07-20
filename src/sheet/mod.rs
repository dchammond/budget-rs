pub mod transaction_sheet;

pub trait Sheet {
    type Row;
    type RowFind;
    type RowHandle;

    #[must_use]
    fn title(&self) -> &str;
    #[must_use]
    fn headers(&self) -> Vec<&str>;
    #[must_use]
    fn lines(&self) -> Vec<String>;
    #[must_use]
    fn insert(&mut self, row: Self::Row);
    #[must_use]
    fn update(&mut self, old_row: &Self::RowFind, new_row: Self::Row);
    #[must_use]
    fn get_row(&self, find: &Self::RowFind) -> Option<Self::RowHandle>;
}
