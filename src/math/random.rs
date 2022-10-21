use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct SimpleDeterministicRandomGenerator
{
    seed: u64
}

impl SimpleDeterministicRandomGenerator
{
    const modulus: u64 = 281_474_976_710_656;
    const multiplier: u64 = 0x5DEECE66D;
    const increment: u64 = 11;

    pub fn new() -> Self
    {
        let time_since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).expect("System clock issue");
        SimpleDeterministicRandomGenerator{seed: time_since_epoch.subsec_nanos() as u64}
    }

    pub fn new_seeded(seed: u64) -> Self
    {
        SimpleDeterministicRandomGenerator{seed}
    }

    pub fn rand(&mut self) -> f64
    {
        let mut rand = 0;
        let mut a = self.seed;
        let mut b = SimpleDeterministicRandomGenerator::multiplier;
        while b > 0
        {
            if b % 2 == 1
            {
                rand = (rand + a) % SimpleDeterministicRandomGenerator::modulus;
            }
            a = (a * 2) % SimpleDeterministicRandomGenerator::modulus;
            b /= 2;
        }
        rand += SimpleDeterministicRandomGenerator::increment;
        self.seed = rand;
        rand as f64 / SimpleDeterministicRandomGenerator::modulus as f64
    }

    pub fn rand_in_range(&mut self, min: u64, max: u64) -> f64
    {
        min as f64 + (max - min) as f64 * self.rand()
    }
}