// use documentation tsts
/// some explanation of the function
/// ```
/// let result = unittest::mul(2,3);
/// assert_eq!(result, 6);
/// ```
pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

/// This function divides two numbers.
/// # Example #1: 10 / 2 == 5
/// ```
/// let result = unittest::div(10, 2);
/// assert_eq!(result, 5);
/// ```
/// # Example #2: 6 / 2 = 3
/// ```
/// let result = unittest::div(6, 2);
/// assert_eq!(result, 3);
/// ```
/// # Panics
/// The function panics if the second argument is zero.
/// ```rust,should_panic
/// // panics on division by zero
/// let result = unittest::div(5, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
/// # Example #1: 9 - 2 == 7
/// ```
/// let result = unittest::sub(9, 2);
/// assert_eq!(result, 7);
/// ```
/// # Example #2: 6 - 9 == -3
/// ```
/// let result = unittest::sub(6, 9);
/// assert_eq!(result, -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

// for integration test
pub struct Pizza {
    pub topping: String,
    pub inches: u8,
}

impl Pizza {
    pub fn pepperoni(inches: u8) -> Self {
        Pizza::bake("pepperoni", inches)
    }

    pub fn mozzarella(inches: u8) -> Self {
        Pizza::bake("mozzarella", inches)
    }

    fn bake(topping: &str, inches: u8) -> Self {
        Pizza {
            topping: String::from(topping),
            inches,
        }
    }
}