// 2 3 2*2*2
// multiply base with itself for exponent time

pub fn pow(base: u8, exponent: usize) -> i64 {
    let mut res = 1;

    if exponent == 0 {
        return 1;
    }

    for _ in 0..exponent {
        res *= base as i64;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::pow;

    #[test]
    fn it_works() {
        assert_eq!(pow(2, 3), 8);
    }
}
