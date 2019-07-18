mod onlookers;
mod primes;
mod protagonists;

fn main() {
    let (prime, root) = primes::generate_prime_and_root();
    let mut public = onlookers::Public {
        prime: prime,
        root: root,
        amy: None,
        ben: None,
    };

    let (amy, ben) = protagonists::generate(public.prime);

    // Mix Amy's secret key with the public prime and
    // root, make the result public.
    public.amy(amy.mix(public.prime, public.root));

    // Mix Ben's secret key with the public prime and
    // root, make the result public.
    public.ben(ben.mix(public.prime, public.root));

    // Mix Amy's secret with Ben's mix; this is kept private.
    let amy_final = amy.mix(public.prime, public.ben.unwrap());

    // Mix Ben's secret with Amy's mix, this is kept private.
    let ben_final = ben.mix(public.prime, public.amy.unwrap());

    println!("Amy = {}", amy_final);
    println!("Ben = {}", ben_final);
    println!("");
    println!(
        "Public = {}, {}, {}, {}",
        public.prime,
        public.root,
        public.amy.unwrap(),
        public.ben.unwrap()
    );
}
