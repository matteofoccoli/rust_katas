fn main() {
    println!("Hello, world!");
}

trait Cupcake {
    fn description(&self) -> String;

    fn has_topping(&self) -> bool;

    fn price(&self) -> f32;
}

trait CupcakeDecorator {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake;
}

struct PlainCupcake {}

impl PlainCupcake {
    fn new() -> Self {
        Self {}
    }
}

impl Cupcake for PlainCupcake {
    fn description(&self) -> String {
        "🧁".to_string()
    }

    fn has_topping(&self) -> bool {
        false
    }
    
    fn price(&self) -> f32 {
        1.0
    }
}

struct ChocolateCupcake {
    cupcake: Box<dyn Cupcake>,
}

impl Cupcake for ChocolateCupcake {
    fn description(&self) -> String {
        if self.cupcake.has_topping() {
            format!("{} and 🍫", self.cupcake.description())
        } else {
            format!("{} with 🍫", self.cupcake.description())
        }
    }

    fn has_topping(&self) -> bool {
        true
    }
    
    fn price(&self) -> f32 {
        0.1 + self.cupcake.price()
    }
}

impl CupcakeDecorator for ChocolateCupcake {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake {
        ChocolateCupcake { cupcake }
    }
}

struct NutsCupcake {
    cupcake: Box<dyn Cupcake>,
}

impl Cupcake for NutsCupcake {
    fn description(&self) -> String {
        if self.cupcake.has_topping() {
            format!("{} and 🥜", self.cupcake.description())
        } else {
            format!("{} with 🥜", self.cupcake.description())
        }
    }

    fn has_topping(&self) -> bool {
        true
    }
    
    fn price(&self) -> f32 {
        0.2 + self.cupcake.price()
    }
}

impl CupcakeDecorator for NutsCupcake {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake {
        NutsCupcake { cupcake }
    }
}

struct CandiesCupcake {
    cupcake: Box<dyn Cupcake>,
}

impl Cupcake for CandiesCupcake {
    fn description(&self) -> String {
        if self.cupcake.has_topping() {
            format!("{} and 🍬", self.cupcake.description())
        } else {
            format!("{} with 🍬", self.cupcake.description())
        }
    }

    fn has_topping(&self) -> bool {
        true
    }
    
    fn price(&self) -> f32 {
        0.7 + self.cupcake.price()
    }
}

impl CupcakeDecorator for CandiesCupcake {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake {
        CandiesCupcake { cupcake }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn creates_a_plain_cupcake() {
        let cupcake = PlainCupcake::new();

        assert_eq!("🧁", cupcake.description());
        assert_eq!("1.00", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn has_chocolate() {
        let cupcake = ChocolateCupcake::new(Box::new(PlainCupcake::new()));

        assert_eq!("🧁 with 🍫", cupcake.description());
        assert_eq!("1.10", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn has_nuts() {
        let cupcake = NutsCupcake::new(Box::new(PlainCupcake::new()));

        assert_eq!("🧁 with 🥜", cupcake.description());
        assert_eq!("1.20", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn has_chocolate_an_nuts() {
        let cupcake = NutsCupcake::new(Box::new(ChocolateCupcake::new(Box::new(
            PlainCupcake::new(),
        ))));

        assert_eq!("🧁 with 🍫 and 🥜", cupcake.description());
        assert_eq!("1.30", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn has_nuts_and_chocolate() {
        let cupcake =
            ChocolateCupcake::new(Box::new(NutsCupcake::new(Box::new(PlainCupcake::new()))));

        assert_eq!("🧁 with 🥜 and 🍫", cupcake.description());
        assert_eq!("1.30", format!("{:.2}", cupcake.price()));
    }

    #[test]
    fn has_nuts_chocolate_and_candies() {
        let cupcake = CandiesCupcake::new(Box::new(ChocolateCupcake::new(Box::new(
            NutsCupcake::new(Box::new(PlainCupcake::new())),
        ))));

        assert_eq!("🧁 with 🥜 and 🍫 and 🍬", cupcake.description());
        assert_eq!("2.00", format!("{:.2}", cupcake.price()));
    }
}
