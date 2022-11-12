# with_unsized_new
A Rust macro for creating constructible unsized (slice-tail) structs.

For example:
```rust
with_unsized_new!{
    #[derive(Debug)]
    struct S{
        a: u16,
        b: u32,
        ~c: [u64]
    }
}
// ...
let usized: Box<S> = S::create_unchecked(1, 2, [33, 44, 55, 66]);
println!("{usized:?}"); // S { a: 1, b: 2, c: [33, 44, 55, 66] }
```

Structs defined in `with_unsized_new` must end in a slice, with that field's name prefixed by `~`. A `create_unchecked` function produces a box of your
(otherwise-unconstructible) type; use it to produce a proper `new` function.
