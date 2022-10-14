use std::{cell::RefCell, thread_local};

/// This creates a new random number based on the data provided.
/// # Arguments:
/// * `min_value`: The minimum value of the random number.
/// * `max_value`: The maximum value of the random number.
/// # Returns:
/// The random number.
/// # Example:
/// ```
/// use random::rand;
///
/// set_seed(1); // Create a new seed.
/// let random_number = rand(0, 100); // Create a random number between 0 and 100.
/// ```
///
pub fn rand(min_value: usize, max_value: usize) -> usize {
    (randomize(8)) % (1 + (max_value - min_value)) + (min_value)
}

thread_local! {
    static SEED: RefCell<u128> = RefCell::new(0);

    static X0: RefCell<u128> = RefCell::new(0);
    static X1: RefCell<u128> = RefCell::new(0);
    static X2: RefCell<u128> = RefCell::new(0);
    static X3: RefCell<u128> = RefCell::new(0);
    static X4: RefCell<u128> = RefCell::new(0);
}

/// Sets the seed for the random number generator.
pub fn set_seed(seed: u128) {
    if seed == 0 {
        return;
    };
    SEED.with(|s| *s.borrow_mut() = seed);
    reset();
}

/// Reset the random number generator.
fn reset() {
    let seed = seed_get();
    let s = [split(seed), split(seed), split(seed), split(seed)];
    put(&s);
}

/// Puts the seed into the registers.
fn put(seeds: &[u128; 4]) {
    X0.with(|s| *s.borrow_mut() = seeds[0]);
    X1.with(|s| *s.borrow_mut() = seeds[1]);
    X2.with(|s| *s.borrow_mut() = seeds[2]);
    X3.with(|s| *s.borrow_mut() = seeds[3]);
}

/// Gets the seed from the registers.
/// # Returns:
/// The seed.
/// # Example:
/// ```
/// use random::gen::seed_get;
/// let seed = seed_get();
/// ```
///
fn seed_get() -> u128 {
    SEED.with(|s| *s.borrow())
}

/// Puts the seed into the registers.
/// # Arguments:
/// * `seed`: The seed to put into the registers.
/// # Example:
/// ```
/// use random::seed_put;
/// seed_put(0);
/// ```
///
///
fn seed_put(seed: u128) {
    SEED.with(|s| *s.borrow_mut() = seed)
}

/// # Credits to: Sebastiano Vigna, 2015
/// # A SPLITMIX64 generator.
/// ## Make a random number from the seed
/// # Arguments:
/// * `seed`: The seed to use
/// # Returns:
/// A random number
/// # Example:
/// ```
/// use random::split;
/// let seed = 123456789;
/// let random = split(seed);
/// ```
/// # Notes:
/// This is a 64-bit version of SplitMix by Sebastiano Vigna , 2015
/// https://xoshiro.di.unimi.it/splitmix64.c
/// This is a fast generator passing BigCrush without systematic failures.
///
fn split(seed: u128) -> u128 {
    let mut s = seed.overflowing_add(0x9e3779b97f4a7c15).0;
    seed_put(s);
    s = (s ^ (s >> 30)).overflowing_mul(0xbf58476d1ce4e5b9).0;
    s = (s ^ (s >> 27)).overflowing_mul(0x94d049bb133111eb).0;
    s ^ (s >> 31)
}

/// Rotates the bits of a number to the left.
/// # Arguments:
/// * `number`: The number to rotate
/// * `rotation`: The number of bits to rotate
/// # Returns:
/// The rotated number
/// # Example:
/// ```
/// use random::rotate;
/// let number = 0b0000000000000000000000000000000000000000000000000000000000000001;
/// let rotation = 1;
/// let rotated = rotate(number, rotation);
/// ```
///
fn rotate(number: u128, rotation: i32) -> u128 {
    number.rotate_left(rotation as u32)
}

/// # Credits to David Blackman and Sebastiano Vigna (vigna@acm.org), 2018.
/// ## This is xoshiro256** 1.0, a 256-bit all-purpose, rock-solid generator.
/// ## It has excellent (about 1ns) speed, a state (256 bits) that is large
/// ## Enough for any parallel application, and it passes all tests we are aware of.
/// ## For generating just floating-point numbers, xoshiro256+ is even faster.
/// ## The state must be seeded so that it is not everywhere zero.
/// ## If you have a 64-bit seed, we suggest to seed a splitmix64 generator and
/// ## use its output to fill s.
///
/// # Returns:
/// A random u128
///
/// # Example:
/// ```
/// use random::xoshiu_64;
/// let x = xoshiu_64();
/// ```
///
/// # See also:
/// [xoshiro256++](https://prng.di.unimi.it/xoshiro256plusplus.c)
/// [xoshiro256+](https://prng.di.unimi.it/xoshiro256plus.c)
/// [xoshiro256**](https://prng.di.unimi.it/xoshiro256starstar.c)
fn randomize(bits: u8) -> usize {
    // get the seeds
    let mut seeds = [
        X0.with(|s| *s.borrow()),
        X1.with(|s| *s.borrow()),
        X2.with(|s| *s.borrow()),
        X3.with(|s| *s.borrow()),
    ];

    // rotate the seeds
    let result = rotate(seeds[1] * 5, 7) * 9;

    let t = seeds[1] << 17;

    // compute new seeds
    seeds[2] ^= seeds[0];
    seeds[3] ^= seeds[1];
    seeds[1] ^= seeds[2];
    seeds[0] ^= seeds[3];
    seeds[2] ^= t;
    seeds[3] = seeds[3].rotate_left(45);

    // put the seeds back / update the thread local seeds
    put(&seeds);

    // return the result
    (result >> (64 - bits)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let number = 0b0000000000000000000000000000000000000000000000000000000000000001;
        let rotation = 1;
        let rotated = rotate(number, rotation);
        assert_eq!(
            rotated,
            0b0000000000000000000000000000000000000000000000000000000000000010
        );
    }

    #[test]
    fn test_randomize() {
        set_seed(123456789);
        let random = randomize(64);
        assert_ne!(random, 0);

        println!("random: {}", random);
    }

    #[test]
    fn test_split() {
        let seed = 123456789;
        let random = split(seed);
        assert_ne!(random, 0);

        println!("split: {}", random);
    }
}
