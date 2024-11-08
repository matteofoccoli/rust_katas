fn main() {
    println!("Hello, world!");
}

trait Cupcake {
    fn print(&self) -> String;
}

trait Topping {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake;
}

struct PlainCupcake {

}

impl Cupcake for PlainCupcake {
    fn print(&self) -> String {
        "🧁".to_string()
    }
}

struct ChocolateCupcake {
    cupcake: Box<dyn Cupcake>,
}

impl Cupcake for ChocolateCupcake {
    fn print(&self) -> String {
        return format!("{} with 🍫", self.cupcake.print())
    }
}

impl Topping for ChocolateCupcake {
    fn new(cupcake: Box<dyn Cupcake>) -> impl Cupcake {
        ChocolateCupcake {
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

        let result = cupcake.print();

        assert_eq!("🧁", result);
    }

    #[test]
    fn puts_some_chocolate_on_top() {
        let cupcake = ChocolateCupcake::new(Box::new(PlainCupcake {}));

        let result = cupcake.print();

        assert_eq!("🧁 with 🍫", result);
    }
}
