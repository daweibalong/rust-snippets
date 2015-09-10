# rust-tools
Some Rust Code Snippets

## cond.rs
rust实现scheme中的cond
* Scheme
```scheme
(let ((a 2))
  (cond 
    ((= a 1) (display "a"))
    ((= a 2) (display "b"))
    ((= a 3) (display "c"))))
```
* Rust
```rust
let a = 2;
cond! {
  a == 1 => {
    println!("a");
  },
  a == 2 => {
    println!("b");
  },
  a == 3 => {
    println!("c");
  }
};

```

## lets.rs
rust实现scheme中的let*
* Scheme
```scheme
(let* ((a 1) (b (+ a 1)))
  b)
```
* Rust
```rust
lets!(a = 1, b = a + 1);
```
