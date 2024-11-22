struct CheckOut {
    pricing_rules: Vec<Box<dyn DiscountRule>>,
    items: Vec<Item>,
}

struct Item {
    sku: String,
    unit_price: f32,
}

struct ItemADiscountRule;

impl DiscountRule for ItemADiscountRule {
    fn calculate(&self, items: &Vec<Item>) -> f32 {
        let discountable_items_count = items.iter().filter(|i| i.sku == "A".to_string()).count();
        ((discountable_items_count / 3) * 20) as f32
    }
}

struct ItemBDiscountRule;

impl DiscountRule for ItemBDiscountRule {
    fn calculate(&self, items: &Vec<Item>) -> f32 {
        let discountable_items_count = items.iter().filter(|i| i.sku == "B".to_string()).count();
        ((discountable_items_count / 2) * 15) as f32
    }
}

trait DiscountRule {
    fn calculate(&self, items: &Vec<Item>) -> f32;
}

impl CheckOut {
    fn new(pricing_rules: Vec<Box<dyn DiscountRule>>) -> Self {
        Self {
            pricing_rules,
            items: vec![],
        }
    }
    fn scan(&mut self, item: Item) {
        self.items.push(item);
    }

    fn total(&self) -> f32 {
        let grand_total: f32 = self.items.iter().map(|i| i.unit_price).sum();
        let discount: f32 = self
            .pricing_rules
            .iter()
            .map(|rule| rule.calculate(&self.items))
            .sum();
        grand_total - discount
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_totals() {
        assert_eq!(0.0, price("".to_string()));
        assert_eq!(50.0, price("A".to_string()));
        assert_eq!(80.0, price("AB".to_string()));
        assert_eq!(115.0, price("CDBA".to_string()));

        assert_eq!(100.0, price("AA".to_string()));
        assert_eq!(130.0, price("AAA".to_string()));
        assert_eq!(180.0, price("AAAA".to_string()));
        assert_eq!(230.0, price("AAAAA".to_string()));
        assert_eq!(260.0, price("AAAAAA".to_string()));

        assert_eq!(160.0, price("AAAB".to_string()));
        assert_eq!(175.0, price("AAABB".to_string()));
        assert_eq!(190.0, price("AAABBD".to_string()));
        assert_eq!(190.0, price("DABABA".to_string()));
    }

    fn price(skus: String) -> f32 {
        let pricing_rules: Vec<Box<dyn DiscountRule>> =
            vec![Box::new(ItemADiscountRule), Box::new(ItemBDiscountRule)];
        let mut check_out = CheckOut::new(pricing_rules);
        skus.chars().for_each(|sku| match sku {
            'A' => check_out.scan(Item {
                sku: "A".to_string(),
                unit_price: 50.0,
            }),
            'B' => check_out.scan(Item {
                sku: "B".to_string(),
                unit_price: 30.0,
            }),
            'C' => check_out.scan(Item {
                sku: "C".to_string(),
                unit_price: 20.0,
            }),
            'D' => check_out.scan(Item {
                sku: "D".to_string(),
                unit_price: 15.0,
            }),
            _ => (),
        });
        check_out.total()
    }
}
