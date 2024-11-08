fn main() {
    println!("Hello, world!");
}

trait Cupcake {
    fn print(&self) -> String;

    fn is_topping(&self) -> bool;
}

trait CupcakeWithTopping {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake;
}

struct PlainCupcake {

}

impl Cupcake for PlainCupcake {
    fn print(&self) -> String {
        "🧁".to_string()
    }
    
    fn is_topping(&self) -> bool {
        false
    }
}

struct ChocolateCupcake {
    cupcake: Box<dyn Cupcake>,
}

impl Cupcake for ChocolateCupcake {
    fn print(&self) -> String {
        if self.cupcake.is_topping() {
            format!("{} and 🍫", self.cupcake.print())
        }
        else {
            format!("{} with 🍫", self.cupcake.print())
        }
    }
    
    fn is_topping(&self) -> bool {
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
    fn print(&self) -> String {
        if self.cupcake.is_topping() {
            format!("{} and 🥜", self.cupcake.print())
        } else {
            format!("{} with 🥜", self.cupcake.print())
        }
    }
    
    fn is_topping(&self) -> bool {
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

        assert_eq!("🧁", cupcake.print());
    }

    #[test]
    fn puts_some_chocolate_on_top() {
        let cupcake = ChocolateCupcake::new(Box::new(PlainCupcake {}));

        assert_eq!("🧁 with 🍫", cupcake.print());
    }

    #[test]
    fn puts_chocolate_an_nuts_on_top() {
        let cupcake = NutsCupcake::new(Box::new(ChocolateCupcake::new(Box::new(PlainCupcake {}))));

        assert_eq!("🧁 with 🍫 and 🥜", cupcake.print());
    }

    #[test]
    fn puts_nuts_and_chocolate_on_top() {
        let cupcake = ChocolateCupcake::new(Box::new(NutsCupcake::new(Box::new(PlainCupcake {}))));

        assert_eq!("🧁 with 🥜 and 🍫", cupcake.print());
    }
}
