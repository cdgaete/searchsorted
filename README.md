## run example

```Bash
cd examples

cargo run --example eg1
```

# searchsorted

Two functions are included cartesian_product and searchsorted:

The idea is the following, we have a table in long format (sample):

| dim1 | dim2 | dim3 | value |
| ---- | ---- | ---- | ----- |
| 'a1' | 'b2' | 'c1' | 10    |
| 'a2' | 'b3' | 'c2' | 20    |

Every dimension has a known domain:

dim1 = {'a1', 'a2'}

dim2 = {'b1','b2','b3'}

dim3 = {'c1','c2'}

I want to create a table with all possible combinations (full) and then allocate the values of the sample table into the full one.

So, my approach is the following:

First step, I want to create an array of tuples (looks like: long format table) with all possible combinations (here, I use the cartesian product function). As second step, I want to find the location of sample array of tuples (table above), so that, later on I can insert the value to the full table.

Step 1: cartesian product (full)

|    | dim1 | dim2 | dim3 |  
| -- | ---- | ---- | ---- | 
|  1 | 'a1' | 'b1' | 'c1' |  
|  2 | 'a1' | 'b1' | 'c2' | 
|  3 | 'a1' | 'b2' | 'c1' |  <!--  first row of sample table above   -->
|  4 | 'a1' | 'b2' | 'c2' | 
|  5 | 'a1' | 'b3' | 'c1' | 
|  6 | 'a1' | 'b3' | 'c2' | 
|  7 | 'a2' | 'b1' | 'c1' | 
|  8 | 'a2' | 'b1' | 'c2' | 
|  9 | 'a2' | 'b2' | 'c1' | 
| 10 | 'a2' | 'b2' | 'c2' | 
| 11 | 'a2' | 'b3' | 'c1' | 
| 12 | 'a2' | 'b3' | 'c2' | <!--  second row of sample table above   -->

Step2: searchsorted

| dim1 | dim2 | dim3 | value | index full table |
| ---- | ---- | ---- | ----- | ---------------- |
| 'a1' | 'b2' | 'c1' | 10    | 3                |
| 'a2' | 'b3' | 'c2' | 20    | 14               |


Summary:
- cartseian product inputs: set of dimensions
- cartesian product returns: an array with tuples
- searchsorted input: full table (array with tuples) and sample table (array with tuples)
- searchsorted returns: an array with integers