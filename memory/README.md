# Memory Management
Rust uses an ownership system to manage memory.
It checks in compile to ensure program efficiency.

## Binding and Dropping
Rust binds variable to values in limited scopes.
This is why variables are unchangable by default.
Scopes include *fu* body and *ifelse* or *match* arms.
When an object goes out of scope, it is dropped.

## Ownership
When a binding exist between a variable and a value:
We can say the variable *owns* the value.
Referring the variable *borrows* the value.

Assigning a variable to another transfers ownership.
ONLY one variable can own a specific value at a time.
This is why we describe binding as a ownership.

## Dropping
Binding is dropped at the boundary of a scope.
Or when the value is moved away from a variable.
Same time, variable and value may also dropped.

After assigning value from one var to another:
We say value is moved from a variable to another.
Cannot borrow value from the first var after move.

## Argument
Passing a variable into the argument of a function:

When the value is String, Vecor, Tuple, Enum, etc:
Rust moves the value from the var into the fu.
Similar to assigning the var to another var.
It means we cannot pass the same var to another fn.

When the value is in simple type like Num or Char:
Rust create a new value or we say copy implicitly.
Because Copy trait is implemented for simple types.

Copying or creating new value is expensive:
Use *clone* method for complex type value with caution.
Or better use *reference* to borrow values from them.

## Borrow
Borrow a var ref does not cause loosing ownership.
Use var ref in fn args if need to use the var later.

Use *mut* for var needs to change value in a fn.
And *&mut* for it in the fn args accordingly.

Compiler does not allow borrowing mutable var twice.
This machenism is designed for eliminating data racing.
One var cannot change at two places at the same time.
Even after immutable borrow, mutable borrow is not allowed.

## Lifetimes
Recall the scope that defines how var and value are dropped.
Borrower var cannot live longer than borrowed value or var.
Or, borrower cannot be used after borrowed item is dropped.

Issue happens when a fn may or not return some var or value:
Since some var or value may get dropped but not returned.
Assign named lifetime param for all arg and returned value.