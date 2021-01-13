fn sum(a: i8, b: i8) -> i8 {
    a + b
}

fn expensive_loop() {
    for _ in 1..1_000_0000 {
        // Do the expensive work
    }
}

#[cfg(test)]
mod my_tests {
    fn sum_input_outputs() -> Vec<((i8, i8), i8)> {
        vec![((1, 1), 2), ((0, 0), 0), ((2, -2), 0)]
    }
    #[test]
    fn test_sum() {
        for (input, output) in sum_input_outputs() {
            assert_eq!(crate::sum(input.0, input.1), output);
        }
    }

    #[test]
    #[should_panic]
    fn panics() {
        assert!(false);
    }

    #[test]
    #[ignore]
    fn test_expensive_fn() {
        crate::expensive_loop();
    }
}
