use std::collections::HashMap;

struct FactorialCalculator {
    known_factorials: HashMap<u128, u128>,
}

impl FactorialCalculator {
    pub fn new() -> FactorialCalculator {
        FactorialCalculator {
            known_factorials: HashMap::from([(0, 1), (1, 1)]),
        }
    }

    pub fn factorial(&mut self, num: u128) -> u128 {
        if let Some(fac) = self.known_factorials.get(&num) {
            return *fac;
        }

        let fac_m1 = self.factorial(num - 1);
        self.known_factorials.insert(num, fac_m1 * num);

        return self.factorial(num - 1) * num;
    }
}

#[cfg(test)]
mod tests {
    use super::FactorialCalculator;

    #[test]
    fn fac_tests() {
        let mut fac_calculator = FactorialCalculator::new();
        assert_eq!(fac_calculator.factorial(0), 1);
        assert_eq!(fac_calculator.factorial(1), 1);
        assert_eq!(fac_calculator.factorial(2), 1 * 2);
        assert_eq!(fac_calculator.factorial(3), 1 * 2 * 3);
        assert_eq!(
            fac_calculator.factorial(9),
            1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9
        );
    }
}
