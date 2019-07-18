extern crate rand;

use rand::Rng;
use std::string::String;

pub struct Person {
    pub name: String,
    secret: u64,
}

impl Person {
    pub fn mix(&self, prime: u64, root: u64) -> u64 {
        return root * self.secret % prime;
    }
}

pub fn generate(prime: u64) -> (Person, Person) {
    let mut rng = rand::thread_rng();

    let amy_name = String::from("Amy");
    let amy_secret: u64 = rng.gen_range(1, prime - 1);
    let amy = Person {
        name: amy_name,
        secret: amy_secret,
    };

    let ben_name = String::from("Ben");
    let ben_secret: u64 = rng.gen_range(1, prime - 1);
    let ben = Person {
        name: ben_name,
        secret: ben_secret,
    };

    return (amy, ben);
}
