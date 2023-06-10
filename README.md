# hashcode

lib

### how-to-use
```toml
[dependencies]
hashcode = { git = "https://github.com/TundraClimate/hashcode.git" }
```

```rust
let hash = HashCode("HashCode").into_u64();

let hash: u64 = HashCode("impl Hash").into();
```