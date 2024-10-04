// https://kata-log.rocks/christmas-lights-kata

fn main() {}

const MAX_LIGHTS: usize = 1000;

#[derive(Clone, Copy)]
enum Status {
    On,
    Off,
}

#[derive(Clone, Copy)]
struct Light {
    status: Status,
}

impl Light {
    fn turn(&mut self, status: Status) {
        self.status = status;
    }

    fn is_on(&self) -> bool {
        if let Status::On = self.status {
            return true;
        }
        false
    }

    fn toggle(&mut self) {
        if let Status::On = self.status {
            self.turn(Status::Off);
        } else {
            self.turn(Status::On);
        }
    }

    fn new() -> Self {
        Self {
            status: Status::Off,
        }
    }
}

struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

struct ChristmasLights {
    // row, columns
    lights: [[Light; MAX_LIGHTS]; MAX_LIGHTS],
}

impl ChristmasLights {
    fn new() -> Self {
        Self {
            lights: [[Light::new(); MAX_LIGHTS]; MAX_LIGHTS],
        }
    }

    fn turn(&mut self, from: Position, to: Position, status: Status) {
        let x_distance = to.x - from.x;
        let y_distance = to.y - from.y;

        for x in from.x..x_distance + 1 {
            for y in from.y..y_distance + 1 {
                self.lights[x][y].turn(status);
            }
        }
    }

    fn toggle(&mut self, from: Position, to: Position) {
        let x_distance = to.x - from.x;
        let y_distance = to.y - from.y;

        for x in from.x..x_distance + 1 {
            for y in from.y..y_distance + 1 {
                self.lights[x][y].toggle();
            }
        }
    }

    fn are_on(&self) -> i32 {
        let mut turned_on = 0;
        for column in &self.lights {
            for i in 0..MAX_LIGHTS {
                let light = &column[i];
                if light.is_on() {
                    turned_on += 1;
                }
            }
        }
        turned_on
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn turns_on_one_light() {
        let mut lights = ChristmasLights::new();

        lights.turn(Position::new(0, 0), Position::new(0, 0), Status::On);

        assert_eq!(1, lights.are_on());
    }

    #[test]
    fn turns_on_two_lights() {
        let mut lights = ChristmasLights::new();

        lights.turn(Position::new(0, 0), Position::new(0, 1), Status::On);

        assert_eq!(2, lights.are_on());
    }

    #[test]
    fn turns_on_four_lights() {
        let mut lights = ChristmasLights::new();

        lights.turn(Position::new(0, 0), Position::new(1, 1), Status::On);

        assert_eq!(4, lights.are_on());
    }

    #[test]
    fn turns_on_nine_lights() {
        let mut lights = ChristmasLights::new();

        lights.turn(Position::new(0, 0), Position::new(2, 2), Status::On);

        assert_eq!(9, lights.are_on());
    }

    #[test]
    fn turns_off_one_light_out_of_one() {
        let mut lights = ChristmasLights::new();
        lights.turn(Position::new(0, 0), Position::new(0, 0), Status::On);

        lights.turn(Position::new(0, 0), Position::new(0, 0), Status::Off);

        assert_eq!(0, lights.are_on());
    }

    #[test]
    fn turns_off_nine_lights_out_of_nine() {
        let mut lights = ChristmasLights::new();
        lights.turn(Position::new(0, 0), Position::new(2, 2), Status::On);

        lights.turn(Position::new(0, 0), Position::new(2, 2), Status::Off);

        assert_eq!(0, lights.are_on());
    }

    #[test]
    fn turns_off_four_lights_out_of_nine() {
        let mut lights = ChristmasLights::new();
        lights.turn(Position::new(0, 0), Position::new(2, 2), Status::On);

        lights.turn(Position::new(0, 0), Position::new(1, 1), Status::Off);

        assert_eq!(5, lights.are_on());
    }

    #[test]
    fn toggles_one_light() {
        let mut lights = ChristmasLights::new();

        lights.toggle(Position::new(0, 0), Position::new(0, 0));

        assert_eq!(1, lights.are_on());
    }

    #[test]
    fn toggles_off_four_lights() {
        let mut lights = ChristmasLights::new();
        lights.turn(Position::new(0, 0), Position::new(2, 2), Status::On);

        lights.toggle(Position::new(0, 0), Position::new(1, 1));

        assert_eq!(5, lights.are_on());
    }

    #[test]
    fn final_test() {
        let mut lights = ChristmasLights::new();

        lights.turn(Position::new(887, 9), Position::new(959, 629), Status::On);
        lights.turn(Position::new(454, 398), Position::new(844, 448), Status::On);
        lights.turn(
            Position::new(539, 243),
            Position::new(559, 965),
            Status::Off,
        );
        lights.turn(
            Position::new(370, 819),
            Position::new(676, 868),
            Status::Off,
        );
        lights.turn(Position::new(145, 40), Position::new(370, 997), Status::Off);
        lights.turn(Position::new(301, 3), Position::new(808, 453), Status::Off);
        lights.turn(Position::new(351, 678), Position::new(951, 908), Status::On);
        lights.toggle(Position::new(720, 196), Position::new(897, 994));
        lights.toggle(Position::new(831, 394), Position::new(904, 860));

        assert_eq!(0, lights.are_on());
    }
}
