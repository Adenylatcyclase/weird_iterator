# Weird Iterator
This crate contains some weird iterators I wrote for practice

## Examples

### Fib
Fibonacci sequence

```rust
// prints 0, 1, 2, 3, 5, 8 ...
// infinitely
for i in Fib::<u32>::new() {
    println!("{}", i);
}

// custom fibonacci sequence
// prints 2, 5, 7, 12 ...
// infinitely
for i in Fib::<u32>::start_at(2, 5) {
    println!("{}", i);
}
```

### FiniteIterator
Turns an infinite iterator into a finite one yielding n elements. May stop yieldine before n is reached if the underlying iterator runs out.
```rust
// prints 0, 1, 2, 3, 5, 8, 13, 21
for i in Fib::<u32>::new().finite(8) {
    println!("{}", i);
}
```


### Spiral
Iterator that yields items from an iterator of iterators in a spiral pattern.
```rust
let s = "abc\ndef\nghi";
// prints adgbehcfi
println!("{}", s.lines().map(|st| st.chars()).spiral().collect::<String>());
```