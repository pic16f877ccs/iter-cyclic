1 # iter-cyclic
  2 ### Rust iterator, range of sequences cyclic with skipping elements
  3  - Creates a new iterator that sequentially outputs a value in the range
  4 with a skip of n elements.
  5 
  6 ### Range
  7  - **start** - the lower bound of the range (inclusive).  
  8  - **end** - the upper bound of the range (inclusive).
  9  - If the start value is greater than the end value, **panic.**
 10    
 11 ### Skip
 12  - skip of n elements.
 13  - **Panic** if value skip conversion to output type error.
 14  
 15 ### Short example                                                                                                                                       
 16 ```
 17    use iter_cyclic::range_skip;
 18    
 19    let vec: Vec<u8> = range_skip(0..5, 200).collect();
 20    assert_eq!(vec, [0, 1, 2, 3, 4, 5, 206, 207, 208, 209, 210, 211]);
 21 
 22 ```
~            
