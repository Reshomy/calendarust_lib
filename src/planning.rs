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

    pub fn items_on(&self, date: chrono::NaiveDate) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| item.occurs_on(date))
            .collect()
    }

    pub fn remove_item(&mut self, index: usize) -> Option<Item> {
        if index >= self.items.len() {
            return None;
        }

        Some(self.items.remove(index))
    }
}
