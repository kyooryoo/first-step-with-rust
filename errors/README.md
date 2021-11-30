# Error Handling

## Panic
The simplest error handling mechanism in Rust.
Use *panic!* thread to panic current thread.
It prints message, free resources, and exit.
Panic should be used for unrecoverable error.

## Option
Rust is explicit about when a value is optional.
Not like other langs take *null* as a string arg.
Rust models an optional string with an enum.
This is already implemented in *Vec::get* method.

## Match
Match compares value and decide following actions.
Match an Option variant to *Some()*, *None*, or *_*.
Put more specific matching arm on top of genearl.
Since *_* matches all possible conditions:
put *Some()* on top of *None*, and *_* at the last.
If nothing to do with matched arm, return *{}*.

## Unwrap
Use *unwrap()* method to extract value from *Some()*.
So *Some(value).unwrap()* returns the inner *value*.
However, *unwrap()* will panic for the *None* value.
Use *unwrap_or(value)* to default *None* to *value*.
Notice, *unwrap* returns a ref such as *&3* but not *3*.

## Expect
The *expect(msg)* method works as the same as *unwrap*.
It provides a custom message *msg* if panic happens.

