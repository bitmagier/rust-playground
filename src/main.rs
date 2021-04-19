const FIRST_PRIME: usize = 2;

fn multiples_of_p(p: usize, limit: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut x = 2;
    let mut r: usize = 0;
    while r <= limit {
        r = p * x;
        if r <= limit {
            result.push(r);
        }
        x += 1;
    };
    result
}

fn primes_sieve_of_eratosthenes(upper_limit: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = vec![true; upper_limit + 1];

    let mut continue_number: Option<usize> = Some(FIRST_PRIME);

    while continue_number.is_some() {
        run_sieve_iteration(&mut sieve, continue_number.expect("invalid state"), upper_limit);
        continue_number = find_smallest_not_marked_number(&sieve, continue_number.expect("invalid state") + 1, upper_limit);
    }

    return resolve_numbers(&sieve);
}

fn find_smallest_not_marked_number(sieve: &Vec<bool>, lower_limit: usize, upper_limit: usize) -> Option<usize> {
    for i in lower_limit..upper_limit {
        if sieve[i] {
            return Some(i);
        }
    }
    None
}

fn run_sieve_iteration(sieve: &mut Vec<bool>, p: usize, upper_limit: usize) {
    for x in multiples_of_p(p, upper_limit).into_iter() {
        sieve[x] = false;
    }
}

fn resolve_numbers(sieve: &Vec<bool>) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();
    for i in FIRST_PRIME..sieve.len() {
        if sieve[i] {
            primes.push(i);
        }
    }
    primes
}


fn main() {
    println!("Hello World!");

    let limit = 100;
    let primes = primes_sieve_of_eratosthenes(limit);
    for p in primes.iter() {
        println!("prime: {}", p)
    }
    println!("total number of primes till {}: {}", limit, primes.len())

}
