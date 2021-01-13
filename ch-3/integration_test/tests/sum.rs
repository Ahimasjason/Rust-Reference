use integration_test::sum;

mod common;

use common::{set_up, tear_down};

#[test]
fn sum_test() {
    assert_eq!(sum(1, 1), 2);
}

#[test]
fn test_with_fixture() {
    set_up();
    assert_eq!(sum(1, 1), 2);
    tear_down();
}
