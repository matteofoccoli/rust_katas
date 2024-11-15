struct CheckOut {}

struct Item {
    sku: String,
    unit_price: f32,
}

impl CheckOut {
    fn scan(&self, item: Item) {

    }

    fn total(&self) -> f32 {
        10.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_item_in_the_cart() {
        let check_out = CheckOut {};

        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});

        assert_eq!(10.0, check_out.total())
    }
}
