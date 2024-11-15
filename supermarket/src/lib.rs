struct CheckOut {
    pricing_rule: PricingRule,
    items: Vec<Item>
}

struct Item {
    sku: String,
    unit_price: f32,
}

struct PricingRule {
    sku: String,
}

impl PricingRule {
    fn discount(&self, items: &Vec<Item>) -> f32 {
        let discountable_items_count = items.iter().filter(|i| i.sku == "A".to_string()).count();
        if discountable_items_count > 1 {
            return 5.0;
        } else {
            return 0.0;
        }
    }
}

impl CheckOut {
    fn new(pricing_rule: PricingRule) -> Self {
        Self {
            pricing_rule,
            items: vec!()
        }
    }
    fn scan(&mut self, item: Item) {
        self.items.push(item);
    }

    fn total(&self) -> f32 {
        let mut total = self.items.iter().map(|i| i.unit_price).sum();
        total -= self.pricing_rule.discount(&self.items);
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_item_in_the_cart() {
        let mut check_out = CheckOut::new(PricingRule { sku: "A".to_string() });

        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});

        assert_eq!(10.0, check_out.total());
    }

    #[test]
    fn two_items_in_the_cart() {
        let mut check_out = CheckOut::new(PricingRule {sku: "A".to_string()});

        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});
        check_out.scan(Item {sku: "B".to_string(), unit_price: 20.0});

        assert_eq!(30.0, check_out.total());
    }

    #[test]
    fn special_offer_on_an_item() {
        let mut check_out = CheckOut::new(PricingRule {sku: "A".to_string()});

        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});
        check_out.scan(Item {sku: "A".to_string(), unit_price: 10.0});

        assert_eq!(15.0, check_out.total());
    }
}
