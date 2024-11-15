struct CheckOut {
    items: Vec<Item>
}

struct Item {
    sku: String,
    unit_price: f32,
}

impl CheckOut {
    fn new() -> Self {
        Self {
            items: vec!()
        }
    }
    fn scan(&mut self, item: Item) {
        self.items.push(item);
    }

    fn total(&self) -> f32 {
        self.items.iter().map(|i| i.unit_price).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_item_in_the_cart() {
        let mut check_out = CheckOut::new();

        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});

        assert_eq!(10.0, check_out.total())
    }

    #[test]
    fn two_items_in_the_cart() {
        let mut check_out = CheckOut::new();

        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});
        check_out.scan(Item {sku: "B".to_string(), unit_price: 20.0});

        assert_eq!(30.0, check_out.total())
    }
}
