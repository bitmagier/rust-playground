use std::fmt;

trait UniverseObject {
    fn number_of_stars(&self) -> u128;
}
struct StarSystem {
    number_of_stars: u128,
}


impl fmt::Display for StarSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(number_of_stars: {})", self.number_of_stars)
    }
}

////////////////////////////////

fn main() {
    let solar_system = StarSystem { number_of_stars: 1 };
    println!("{}", solar_system.to_string());
    println!("{}", solar_system)
}