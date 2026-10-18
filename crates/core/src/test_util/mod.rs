use rand::prelude::*;

pub trait RandomValue: PartialEq {
    fn random_value() -> Self;
}

pub trait RandomValueVec: PartialEq {
    fn random_n_values(n: usize) -> Self;
}

impl<T> RandomValueVec for Vec<T>
where
    T: RandomValue,
{
    fn random_n_values(n: usize) -> Self {
        let mut result = Vec::new();
        for _ in 0..n {
            let mut value: T;
            loop {
                value = RandomValue::random_value();
                if !result.contains(&value) {
                    break;
                }
            }
            result.push(value);
        }
        result
    }
}

impl<T> RandomValue for Vec<T>
where
    T: RandomValue,
{
    fn random_value() -> Self {
        Self::random_n_values(30)
    }
}

impl<T> RandomValue for Option<T>
where
    T: RandomValue,
{
    fn random_value() -> Self {
        if RandomValue::random_value() {
            Some(RandomValue::random_value())
        } else {
            None
        }
    }
}

impl RandomValue for bool {
    fn random_value() -> Self {
        let mut rng = rand::rng();
        rng.random_bool(0.5)
    }
}

impl RandomValue for i64 {
    fn random_value() -> Self {
        let mut rng = rand::rng();
        rng.random_range(i64::MIN..i64::MAX)
    }
}

impl RandomValue for String {
    fn random_value() -> Self {
        let mut rng = rand::rng();
        let count = rng.random_range(30..100);
        rng.sample_iter(rand::distr::Alphanumeric)
            .take(count)
            .map(char::from)
            .collect()
    }
}
