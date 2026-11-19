use crate::items::Item;

#[derive(Default)]
pub struct Planning {
    items: Vec<Item>,
}

impl Planning {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn items(&self) -> &[Item] {
        &self.items
    }
}
