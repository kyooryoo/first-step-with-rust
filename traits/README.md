# Generic Types and Traits

## Generic Types
Defined in terms of other types, partially unknown.
We can specify operation no matter the inner types.
Declare type param inside angle brackets after struct.
Then use the type in struct or specify other types.

Compiler can determine a proper type from input value.
Be careful using one generic type for multiple fields.
If defined like this, all fields must be in one type.
Or define each fields with specific generic types.

## Traits
An interface with methods that defined for unkonwn types.
The procedure of implementing a trait looks like:
1. Define the trait which defines input and output;
2. Define the structs that may have verious fields;
3. Implement the trait for these structs;
4. Create struct objects and call implemented methods;

The steps above are identical to interface in other langs.
Notice, traits accept unkonwn typed input in step 3 above.
Because implementation handles different fields per struct.

## Derive
Rust can implement some traits automatically with *Derive*.
For example, for printing out struct object content:
We can add `[derive(Debug)]` on top of struct declaration.
Similary, for checking two struct obs are identical or not:
We can add `[derive(PartialEq)]` to struct declaration.

Otherwise, compiler will complain during running:
*`Trait<T, U>` doesn't implement `Debug`*
*`Trait<T, U>` cannot be formatted using `{:?}`*

## Trait Bounds
Implementing a trait as an object method is one option.
Another option is creating generic function alike thing.
This hides obj methods and makes the fn looks generic.
-- Struct specific trait implementation is hidden.

Besides standard trait impl procedure, add one more step:
* define the generic function takes *&impl Trait* as arg
Rust actually takes struct obj as the function argument.
And will call the implemented obj method inside the fn.

Trait bounds is one step further of trait implementation.
It wraps implemented trait and obj methods in another fn.

## Iterator
Rust implements Iterator trait with many default methods.
We can override it with self defined methods like *next()*.
And it returns Option type values of *None* or *Some(value)*.
Return value with `Some(value)` or `return None;` explicitly.
See sample and exercise code for Iterator implementations.
