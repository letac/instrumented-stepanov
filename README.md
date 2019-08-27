# instrumented-stepanov

InstrumentedBase is collecting data about number of new, clone, drop, eq, partial cmp and cmp from Instrumented instates.

# Example 

```
let n = count_operations(vec![2, 1, 3, 4], |x| x.sort());
println!("{:?}", n);
```

Attempt to do [Efficient Programming with Components: Lecture 3 Part 1](https://www.youtube.com/watch?v=sp_IBYVqMeQ) and [Efficient Programming with Components: Lecture 3 Part 2](https://www.youtube.com/watch?v=VelLby6K2jQ) from C++ to Rust.
