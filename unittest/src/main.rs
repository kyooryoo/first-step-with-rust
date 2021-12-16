fn main() {
    println!("Hello, unittest!");
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn add_works() {
    assert_eq!(add(1, 3), 4);
    assert_eq!(add(10, 12), 22);
    assert_eq!(add(5, -2), 3);
}

#[test]
#[should_panic]
fn add_fails() {
    assert_eq!(add(1, 3), 5);
}

#[test]
#[ignore = "not yet reviewed"]
fn add_negs() {
    assert_eq!(add(-1, -2), -3);
}

#[cfg(test)]
mod add_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 3), 4);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(1, 3), 5);
    }

    #[test]
    #[ignore = "not yet reviewed"]
    fn add_negs() {
        assert_eq!(add(-1, -2), -3);
    }
}

// an exercise with unit tests
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(2));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(1));
    }

    #[test]
    #[should_panic]
    fn or_fail_when_odd() {
        assert!(is_even(1));
    }
}