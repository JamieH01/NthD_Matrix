# Documentation
## matrix!
A macro that makes initializing matrixes easier. General warning that as larger matrixes are made, the size of the vector can get extremely large. Optional parameter at the end for multithreading, as they can take a while to initialize if theyre very big. Recommend setting to 4-8, 16 if you want.
```
//dimensions, type, default, threads
let matrix = matrix!([3,4]; usize, 15, 8);
```

## NdMatrix<T>
The matrix struct.
### fields
```
pub data:Vec<T>,//data of the matrix. beware changing this directly.

dimensions:usize,//how many dimensions. this can be grabbed with the dim() method.
size:Vec<usize>,//size of each dimension. this can be grabbed with the size() method.
length:usize,//length of the vector. this can be grabbed with the len() method.
```

### Methods
`new(dim, default, threads)`: Creates a matrix. generally advised to use the macro instead. Multithreading support for initialization.
```
let matrix = NdMatrix::<u32>::new(vec![4,3,5], 15, 1);
```

`pos(vec)`: Gets the value at a given position. Returns a Result.
```
let value = matrix.pos(vec![3,2,4])?;
```
`set_pos(vec, val)`: Sets the value at a given position. Returns a Result, however the "happy" path contains no meaningful data.
```
matrix.set_pos(vec![2,2,2], 30);
```

`nth(i)`: Gets the value at a given index of the vector. Returns a Result.
```
let value = matrix.nth(15)?;
```
`set_nth(i, val)`: Sets the value at a given index of the vector. Returns a Result, however the "happy" path contains no meaningful data.
```
matrix.set_nth(10, 25);
```

`pos_to_nth`: Converts a position to an index in the vector. Returns a Result.
```
let pos_to_index = matrix.pos_to_nth(vec![4,3])?;
```
`nth_to_pos`: Converts an index to its corresponding position in the matrix. Returns a Result.
```
let index_to_pos:Vec<usize> = matrix.nth_to_pos(25)?;
```

### Arithmetic
`add`, `sub`, `mul`, and `div` are implemented for types that fulfill the `num_traits::Num` constraint. Constant variants are also available. Note that `mul` is not the standard "matrix multiplication," rather just multiplying each element together.
```
let mut matrix_a = matrix!([5,5]; usize, 10);
let matrix_b = matrix!([5,5]; usize, 10);

matrix_a.add(matrix_b);//adds 2 matrixes
matrix_a.const_add(15);//adds one value to every element of a matrix
```

### Iterators
NdMatrix has an interator implemented. It returns the value, index, and position.
```
let matrix = matrix!([3,3]; i32, 0);
let mapped = &matrix.into_iter().map(|(i,n,p)| n).collect::<Vec<_>>();
```
