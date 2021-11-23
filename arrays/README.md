# Array and Vector
They both hold elements in the same type.
Array has fixed size, vector can change.
They handle *index out of bounds* differently.


## Array
A collection of objects in the same type.
Items are stored sequentially in memory.

Could be defined by specifying items.
Or an initial value and array length.

Array has its signature as *[T; size]*:
* *T* is the data type for all elements
* *size* is the array length
Data type and array length cannot change.
Only the element values can change.

Accesse elements with *<array>[<index>]*
Index value starts from *0* as the first.
Take care of *index out of bounds* error.

## Vector
Vector elements are in the same data type.
However, vector's size or length can change.

Watch out for *insex out of bounds* error:
It now happens at runtime, not compile time.

Vector has extra *push* and *pop* methods.

