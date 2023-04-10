use rand::Rng;

pub fn random_series() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(5..12)
}

pub fn calculate_total_weight(weight: i32, reps: i32, series: i32) -> i32 {
    weight * reps * series
}

#[cfg(test)]
mod tests {
    use super::calculate_total_weight;

    #[test]
    fn test_calculate_total_weight() {
        assert_eq!(calculate_total_weight(90, 6, 5), 2700);
    }
}
