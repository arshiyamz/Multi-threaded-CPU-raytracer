use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct SimpleDeterministicRandomGenerator
{
    seed: u64
}

impl SimpleDeterministicRandomGenerator
{
    const MODULUS: u64 = 281_474_976_710_656;
    const MULTIPLIER: u64 = 0x5DEECE66D;
    const INCREMENT: u64 = 11;

    pub fn new() -> Self
    {
        let time_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("System clock issue");
        let mut time_in_ns = time_since_epoch.subsec_nanos() as u64;
        time_in_ns = time_in_ns * time_in_ns;
        let mut seed = time_in_ns % 10;
        while time_in_ns > 0
        {
            time_in_ns /= 10;
            seed = seed * 10 + (time_in_ns % 10)
        }
        seed = seed % Self::MODULUS;
        SimpleDeterministicRandomGenerator{seed}
    }

    pub fn new_seeded(seed: u64) -> Self
    {
        SimpleDeterministicRandomGenerator{seed}
    }

    // Returns a random number between 0 and 1
    pub fn rand(&mut self) -> f64
    {
        let mut rand = 0;
        let mut a = self.seed;
        let mut b = SimpleDeterministicRandomGenerator::MULTIPLIER;
        while b > 0
        {
            if b % 2 == 1
            {
                rand = (rand + a) % SimpleDeterministicRandomGenerator::MODULUS;
            }
            a = (a * 2) % SimpleDeterministicRandomGenerator::MODULUS;
            b /= 2;
        }
        rand += SimpleDeterministicRandomGenerator::INCREMENT;
        self.seed = rand;
        rand as f64 / SimpleDeterministicRandomGenerator::MODULUS as f64
    }

    pub fn rand_in_range(&mut self, min: u64, max: u64) -> f64
    {
        min as f64 + (max - min) as f64 * self.rand()
    }

    pub fn rand_between(&mut self, min: f64, max: f64) -> f64
    {
        min + (max - min) * self.rand()
    }
}