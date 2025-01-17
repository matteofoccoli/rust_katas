pub trait Pastry {
    fn description(&self) -> String {
        self.symbol().to_string()
    }

    fn symbol(&self) -> char;

    fn has_topping(&self) -> bool {
        false
    }

    fn price(&self) -> f32;
}

pub trait Topping {
    fn new(pastry: Box<dyn Pastry>) -> impl Pastry;
}

pub struct Cupcake {}

impl Cupcake {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pastry for Cupcake {
    fn symbol(&self) -> char {
        '🧁'
    }

    fn price(&self) -> f32 {
        1.0
    }
}

pub struct Cookie {}

impl Cookie {
    pub fn new() -> Self {
        Self {}
    }
}

impl Pastry for Cookie {
    fn symbol(&self) -> char {
        '🍪'
    }

    fn price(&self) -> f32 {
        2.0
    }
}

struct Chocolate {
    pastry: Box<dyn Pastry>,
}

impl Pastry for Chocolate {
    fn description(&self) -> String {
        if self.pastry.has_topping() {
            format!("{} and {}", self.pastry.description(), self.symbol())
        } else {
            format!("{} with {}", self.pastry.description(), self.symbol())
        }
    }

    fn symbol(&self) -> char {
        '🍫'
    }

    fn has_topping(&self) -> bool {
        true
    }

    fn price(&self) -> f32 {
        0.1 + self.pastry.price()
    }
}

impl Topping for Chocolate {
    fn new(pastry: Box<dyn Pastry>) -> impl Pastry {
        Chocolate { pastry }
    }
}

struct Nuts {
    pastry: Box<dyn Pastry>,
}

impl Pastry for Nuts {
    fn description(&self) -> String {
        if self.pastry.has_topping() {
            format!("{} and {}", self.pastry.description(), self.symbol())
        } else {
            format!("{} with {}", self.pastry.description(), self.symbol())
        }
    }

    fn symbol(&self) -> char {
        '🥜'
    }

    fn has_topping(&self) -> bool {
        true
    }

    fn price(&self) -> f32 {
        0.2 + self.pastry.price()
    }
}

impl Topping for Nuts {
    fn new(pastry: Box<dyn Pastry>) -> impl Pastry {
        Nuts { pastry }
    }
}

struct Candies {
    pastry: Box<dyn Pastry>,
}

impl Pastry for Candies {
    fn description(&self) -> String {
        if self.pastry.has_topping() {
            format!("{} and {}", self.pastry.description(), self.symbol())
        } else {
            format!("{} with {}", self.pastry.description(), self.symbol())
        }
    }

    fn symbol(&self) -> char {
        '🍬'
    }

    fn has_topping(&self) -> bool {
        true
    }

    fn price(&self) -> f32 {
        0.7 + self.pastry.price()
    }
}

impl Topping for Candies {
    fn new(pastry: Box<dyn Pastry>) -> impl Pastry {
        Candies { pastry }
    }
}

pub struct Bundle {
    pastries: Vec<Box<dyn Pastry>>,
}

impl Bundle {
    pub fn new() -> Self {
        Self { pastries: vec![] }
    }

    pub fn add_pastry(&mut self, pastry: Box<dyn Pastry>) {
        self.pastries.push(pastry);
    }

    pub fn add_bundle(&mut self, bundle: Bundle) {
        for pastry in bundle.pastries {
            self.pastries.push(pastry)
        }
    }

    pub fn price(&self) -> f32 {
        self.pastries.iter().map(|x| x.price() * 0.9).sum()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn creates_a_cupcake() {
        let cupcake = Cupcake::new();

        assert_eq!("🧁", cupcake.description());
        assert_eq!("1.00", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn creates_a_cookie() {
        let cookie = Cookie::new();

        assert_eq!("🍪", cookie.description());
        assert_eq!("2.00", format!("{:.2}", cookie.price()));
    }

    #[test]
    fn a_cupcake_with_chocolate() {
        let cupcake = Chocolate::new(Box::new(Cupcake::new()));

        assert_eq!("🧁 with 🍫", cupcake.description());
        assert_eq!("1.10", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn a_cookie_with_chocolate() {
        let cookie = Chocolate::new(Box::new(Cookie::new()));

        assert_eq!("🍪 with 🍫", cookie.description());
        assert_eq!("2.10", format!("{:.2}", cookie.price()));
    }

    #[test]
    fn a_cupcake_with_nuts() {
        let cupcake = Nuts::new(Box::new(Cupcake::new()));

        assert_eq!("🧁 with 🥜", cupcake.description());
        assert_eq!("1.20", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn a_cupcake_with_chocolate_an_nuts() {
        let cupcake = Nuts::new(Box::new(Chocolate::new(Box::new(Cupcake::new()))));

        assert_eq!("🧁 with 🍫 and 🥜", cupcake.description());
        assert_eq!("1.30", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn a_cupcake_with_nuts_and_chocolate() {
        let cupcake = Chocolate::new(Box::new(Nuts::new(Box::new(Cupcake::new()))));

        assert_eq!("🧁 with 🥜 and 🍫", cupcake.description());
        assert_eq!("1.30", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn a_cupcake_with_nuts_chocolate_and_candies() {
        let cupcake = Candies::new(Box::new(Chocolate::new(Box::new(Nuts::new(Box::new(
            Cupcake::new(),
        ))))));

        assert_eq!("🧁 with 🥜 and 🍫 and 🍬", cupcake.description());
        assert_eq!("2.00", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn bundle() {
        let plain_cupcake = Cupcake::new();
        let choco_cupcake = Chocolate::new(Box::new(Cupcake::new()));

        let mut bundle = Bundle::new();
        bundle.add_pastry(Box::new(plain_cupcake));
        bundle.add_pastry(Box::new(choco_cupcake));

        assert_eq!(
            format!("{:.2}", (1.0 * 0.9) + (1.1 * 0.9)),
            format!("{:.2}", bundle.price())
        );
    }

    #[test]
    fn bundle_of_bundles() {
        let mut first_bundle = Bundle::new();
        first_bundle.add_pastry(Box::new(Cupcake::new()));
        first_bundle.add_pastry(Box::new(Chocolate::new(Box::new(Cupcake::new()))));

        let mut second_bundle = Bundle::new();
        second_bundle.add_pastry(Box::new(Candies::new(Box::new(Cupcake::new()))));
        second_bundle.add_pastry(Box::new(Chocolate::new(Box::new(Cupcake::new()))));

        let mut bundle_of_bundles = Bundle::new();
        bundle_of_bundles.add_bundle(first_bundle);
        bundle_of_bundles.add_bundle(second_bundle);

        let first_bundle_expected_price = (1.0 * 0.9) + (1.1 * 0.9);
        let second_bundle_expected_price = (1.7 * 0.9) + (1.1 * 0.9);
        assert_eq!(
            format!(
                "{:.2}",
                first_bundle_expected_price + second_bundle_expected_price
            ),
            format!("{:.2}", bundle_of_bundles.price())
        );
    }
}

fn main() {
    print!("Hello world");
}
