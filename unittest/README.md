# Unit Test
Unit test uses *#[test]* makred function for testing.
The function for test usually does not return anything.
Use `assert_eq!(<function>(), <result>);` for testing.
Execute the unit test with command `cargo test`.

## Attribute
We can add the following attribute for special purpose:
* #[should_panic] for testing failed scenario
* #[ignore = "reason"] for omit test with some reason
* #[cfg(test)] for conditional compilation
*should_panic* and *ignore* are addtional attributes.
They are added after the *test* function attribute.

Use *cfg(test)* for making a module as for testing.
Following module only compile with *cargo test* flag.
For access outer fn `use super::*;` in testing mod.

## Documentation test
Documentation test is ONLY for library crates file.
Use *///* for each line of doc line and testing code.
Notice, the main source file locates at *src/main.rs*.
While the library source file locates at *src/lib.rs*.

For fail or panic test the first code line should be:
*```rust,should_panic*
It tells compiler that following test will cause panic.
This is equivalent to the *#[should_panic]* attribute.

## Integration test
We can put all testing code files in a seperate folder.
Create *tests* folder aside *src* for holding test files.
We can have mutiple testing files with meaningful names.
