pub fn clamp<T>(input: T, min: T, max: T) -> T where
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
