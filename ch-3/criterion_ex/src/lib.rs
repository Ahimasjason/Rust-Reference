pub fn fib_slow(n: usize) -> u64 {
    if n <= 1 {
        return n as u64;
    }

    return fib_slow(n - 1) + fib_slow(n - 2);
}

pub fn match_fibnacci(n: u64) -> u64 {
    match n {
        0 => n,
        1 => n,
        _ => match_fibnacci(n - 1) + match_fibnacci(n - 2),
    }
}

// Inspiration from SCIP
pub fn recursive_process_fibancci(n: u64, prev_num: u64, res: u64) -> u64 {
    if n <= 1 {
        return res;
    }
    return recursive_process_fibancci(n - 1, res, res + prev_num);
}

pub fn recursive_proceudre_fib(nth: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut res = 0;
    for _ in 1..nth {
        res = a + b;
        a = b;
        b = res;
    }
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn slow_test() {
        assert_eq!(13, crate::fib_slow(7));
    }
    #[test]
    fn match_test() {
        assert_eq!(13, crate::match_fibnacci(7));
    }

    #[test]
    fn rcr_pr_test() {
        assert_eq!(13, crate::recursive_process_fibancci(7, 0, 1));
    }

    #[test]
    fn rcr_proc_test() {
        assert_eq!(13, recursive_proceudre_fib(7));
    }
}
