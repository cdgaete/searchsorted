## run example

```Bash
cd examples

cargo run --example eg1
```

# searchsorted

code repository here: https://github.com/cdgaete/searchsorted

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
| 'a2' | 'b3' | 'c2' | 20    | 12               |


Summary:
- cartseian product inputs: set of dimensions
- cartesian product returns: an array with tuples
- searchsorted input: full table (array with tuples) and sample table (array with tuples)
- searchsorted returns: an array with integers

## Functions

Cartesian product:

```Rust
type TS3 = (String,String,String);

pub fn cartesian_3d(l1: Vec<String>, l2: Vec<String>, l3: Vec<String>) -> Vec<TS3> {
    let mut collector = Vec::new();
    for tuple in iproduct!(l1,l2,l3) {
        collector.push(tuple);
    };
    collector
}
```

searchsorted:

```Rust
type TS3 = (String,String,String);

pub fn searchsorted_3d(dense_list: Vec<TS3>, index_list: Vec<TS3>) -> Vec<i64> {
    let mut htbl = HashMap::new();
    let mut i: i64 = 0;
    for key in dense_list.iter() {
        htbl.insert(key, i);
        i += 1i64
    };
    let mut location: Vec<i64> = Vec::new();
    for tuple in index_list.iter() {
        let value = htbl.get(tuple).unwrap();
        location.push(*value);
    };
    location
}
```

## Benchmark

In examples folder eg2.py and eg2.rs contain the benchmark code:

- full vector: a million tupples of five string each.
- sample vector: 1000 tuples
- each string in a tuple has 2 chars

Results:

Cartesian product:

```bash
Rust-python   eTime: 1342697 μs.
Pure Rust     eTime   246470 μs.
Pure python   eTime:   84879 μs.
```

searchsorted:

```bash
Rust-python   eTime: 2599270 μs.
Pure Rust     eTime  2015062 μs.
Pure python   eTime:  103814 μs.
```

Code for pure Python:
---------------------

Cartesian product: Itertools package

```Python
list(itertools.product(lst1,lst2,lst3,lst4,lst5))
```

searchsorted: dictionary and list comprension

```Python
def pysearchsorted(full_list, sample_list):
    fullhashtable = {tupl: idx for idx, tupl in enumerate(full_list)}
    return [fullhashtable[tupl] for tupl in sample_list]
```