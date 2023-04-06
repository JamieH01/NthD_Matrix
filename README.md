A simple library that allows you to use matrixes of any size and dimension, and access their internals as a regular old vector. NdMatrix provides a flexible data structure for any data, with basic arithmetic operations (thanks num_traits!).

Matrixes can be initialized easily with the `matrix!` macro:
```
//dimensions, type, default
let matrix = matrix!([10,10,10]; usize, 15);
```
And then accessed with either coordinates or an absolute index into the vector:
```
let value1 = matrix.pos(vec![3,2,4])?;
let value2 = matrix.nth(15)?;
```
This setup makes iteration through a matrix straightforward, with no messy nested for loops:
```
for i in 0..matrix.len() {
    let pos:Vec<usize> = matrix.nth_to_pos(i)?;
    //do stuff with the position!
}
```

Basic arithmetic is also available for numeric types:
```
let mut matrix_a = matrix!([5,5]; usize, 10);
let matrix_b = matrix!([5,5]; usize, 10);

matrix_a.add(matrix_b);//adds 2 matrixes
matrix_a.const_add(15);//adds one value to every element of a matrix
```
`add`, `sub`, `div`, and `mul` can be called, along with their constant counterparts. Note that 2 matrixes added MUST be the same size, and `mul` is not the traditional "matrix multiplication," it simply multiplies each element together (coming soon).