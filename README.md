# iter-cyclic

#### Rust iterator, range of sequences cyclic with skipping elements
 - Creates a new iterator that sequentially outputs a value in the range
with a skip of n elements.

#### Range
 - **start** - the lower bound of the range (inclusive).  
 - **end** - the upper bound of the range (inclusive).
 - If the start value is greater than the end value, **panic.**
   
#### Skip
 - skip of n elements.
 - **Panic** if value skip conversion to output type error.
 
#### Cycling 
 - cycling to the maximum possible value of the range type 
 
## Usage

#### Add this to your Cargo.toml
```
[dependencies]
iter-cyclic = { git = "https://github.com/pic16f877ccs/iter-cyclic", version = "0.1.0" }
```
#### Or using cargo
```
cargo add iter-cyclic --git "https://github.com/pic16f877ccs/iter-cyclic"
```
#### Example
```rust
use iter_cyclic::range_skip;
   
let vec: Vec<u8> = range_skip(0..5, 200).collect();
assert_eq!(vec, [0, 1, 2, 3, 4, 5, 206, 207, 208, 209, 210, 211]);

```
        
