trait Currency {
    fn empty() -> Self;

    fn new(value: f32) -> Self;

    fn value(&self) -> f32;
}

#[derive(PartialEq, Debug)]
struct Eur {
    value: f32,
}

#[derive(PartialEq, Debug)]
struct Usd {
    value: f32,
}

#[derive(PartialEq, Debug)]
struct Gbp {
    value: f32,
}

impl Currency for Eur {
    fn empty() -> Self {
        Self { value: 0.0 }
    }

    fn new(value: f32) -> Self {
        Eur { value }
    }

    fn value(&self) -> f32 {
        self.value
    }
}

impl Currency for Usd {
    fn empty() -> Self {
        Self { value: 0.0 }
    }

    fn new(value: f32) -> Self {
        Usd { value }
    }

    fn value(&self) -> f32 {
        self.value
    }
}

impl Currency for Gbp {
    fn empty() -> Self {
        Self { value: 0.0 }
    }

    fn new(value: f32) -> Self {
        Gbp { value }
    }

    fn value(&self) -> f32 {
        self.value
    }
}

struct ExchangeDesk;

impl ExchangeDesk {
    fn convert<T, K>(from: T, to: K, exchange_rate: f32) -> K
    where
        T: Currency,
        K: Currency,
    {
        let value = exchange_rate * from.value();
        K::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eur_to_usd() {
        let dollars = ExchangeDesk::convert(Eur::new(100.0), Usd::empty(), 1.0423);

        assert_eq!("104.23", format!("{:.2}", dollars.value));
    }

    #[test]
    fn eur_to_gbp() {
        let pounds = ExchangeDesk::convert(Eur::new(1.0), Gbp::empty(), 0.8328);

        assert_eq!("0.83", format!("{:.2}", pounds.value));
    }
}
