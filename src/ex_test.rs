/// This function adds 2 numbers.
///
/// # Example
///
/// ```
/// use ex_test::add;
///
/// add(1 ,2)
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test_add() {
    assert_eq!(0, add(0, 0));
    assert_eq!(1, add(1, 0));
    assert_eq!(1, add(0, 1));
    assert_eq!(2, add(1, 1));
}

#[test]
fn assert_sample() {
    assert!(true);

    // assert!(false, "panic! value={}", false); // it will be failed.

    assert_eq!(true, true);
    //assert_eq!(true, false); // it will be failed.

    // assert_eq!(true, false, "panic! value=({}, {})", true, false); // it will be failed.
}

#[test]
#[should_panic]
fn test_panic() {
    panic!("expected panic");
}

#[test]
#[ignore]
fn test_add_ignored() {
    // test with cargo test -- --ignored
    assert_eq!(-2, add(-1, -1));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[cfg(test)]
mod tests_2 {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}

// use test_code::add;
// #[test]
// fn integration_test() {
//    assert_eq!(3, add(1, 2));
// }

