fn main() {
    println!("Hello, world!");
}

trait Cupcake {
    fn description(&self) -> String;

    fn has_topping(&self) -> bool;
}

trait CupcakeWithTopping {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake;
}

struct PlainCupcake {

}

impl Cupcake for PlainCupcake {
    fn description(&self) -> String {
        "🧁".to_string()
    }
    
    fn has_topping(&self) -> bool {
        false
    }
}

struct ChocolateCupcake {
    cupcake: Box<dyn Cupcake>,
}

impl Cupcake for ChocolateCupcake {
    fn description(&self) -> String {
        if self.cupcake.has_topping() {
            format!("{} and 🍫", self.cupcake.description())
        }
        else {
            format!("{} with 🍫", self.cupcake.description())
        }
    }
    
    fn has_topping(&self) -> bool {
        true
    }
}

impl CupcakeWithTopping for ChocolateCupcake {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake {
        ChocolateCupcake {
            cupcake
        }
    }
}

struct NutsCupcake {
    cupcake: Box<dyn Cupcake>
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
}

impl CupcakeWithTopping for NutsCupcake {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake {
        NutsCupcake {
            cupcake
        }
    }
}
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn creates_a_plain_cupcake() {
        let cupcake = PlainCupcake {};

        assert_eq!("🧁", cupcake.description());
    }

    #[test]
    fn has_chocolate_on_top() {
        let cupcake = ChocolateCupcake::new(Box::new(PlainCupcake {}));

        assert_eq!("🧁 with 🍫", cupcake.description());
    }

    #[test]
    fn has_nuts_on_top() {
        let cupcake = NutsCupcake::new(Box::new(PlainCupcake {}));

        assert_eq!("🧁 with 🥜", cupcake.description());
    }

    #[test]
    fn has_chocolate_an_nuts_on_top() {
        let cupcake = NutsCupcake::new(Box::new(ChocolateCupcake::new(Box::new(PlainCupcake {}))));

        assert_eq!("🧁 with 🍫 and 🥜", cupcake.description());
    }

    #[test]
    fn has_nuts_and_chocolate_on_top() {
        let cupcake = ChocolateCupcake::new(Box::new(NutsCupcake::new(Box::new(PlainCupcake {}))));

        assert_eq!("🧁 with 🥜 and 🍫", cupcake.description());
    }
}
