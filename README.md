# unrelatable
a relational algebra system for data collections

### discussion

i originally wanted to implement this as a GADT, something like:

```rust

enum Row<K: Eq + Clone, L1, L2, R1, R2> {
  SimpleRow {
    key: K
    value: L1
  }
  LeftJoinRow {
    key: K,
    left: Row<L1, L2>,
    right: Option<Row<R1, R2>>
  }
}
```

i had trouble with that, as it recurses, rust gets unhappy.

so i move to trait objects:

```rust
pub trait RowLike<K: Eq + Clone, L, R> {
    fn get_key(&self) -> &K;
    fn get_left(&self) -> Option<&Box<&L>>;
    fn get_right(&self) -> Option<&Box<&R>>;
}
```

but that would move everything onto the heap. even though performance isn't necessarily my main goal here, it seems silly to move all of these operations to the heap.

so i went back to the GADT idea. but then i went back to the heap idea, this time leveraging the pattern for trampolining/recursion schemes from the recursion library.

but that felt too experimental for rust. the library is simple, doesn't add a ton on top, but also there's no mention of benchmarks/performance. it seems to do a lot of extra vector building in the trampoline step.

so, idioms. maybe the right way to go about this is something more c/zig/nim-like that would require me to deal with more state management in the control flow, but run on the stack and not get tied up in the types.