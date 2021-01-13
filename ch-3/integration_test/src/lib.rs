#![doc(html_logo_url = "https://cdn.vox-cdn.com/uploads/chorus_asset/file/10139041/pepe.jpg")]
//! This crate provides functionality for adding numbers
//! # Example
//!```
//!use integration_test::sum;
//!let a = 2;
//!let b = 4;
//!let ans = sum(a,b);
//!```

pub fn sum(a: i8, b: i8) -> i8 {
    a + b
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, crate::sum(2, 2));
    }
}
