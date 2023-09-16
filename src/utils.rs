use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Distribution, Uniform};
use std::ops::RangeInclusive;

pub fn clamp<T>(input: T, min: T, max: T) -> T
where
    T: PartialOrd<T>,
{
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

pub fn random<T>(r: RangeInclusive<T>) -> T
where
    T: SampleUniform,
{
    let mut rng = rand::thread_rng();
    let dist = Uniform::from(r);
    dist.sample(&mut rng)
}
