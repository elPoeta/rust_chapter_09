# Rust Chapter IX

if you want to abort on panic in release mode, add this in your Cargo.toml file:

```
[profile.release]
panic = 'abort'
```

panic backtrace command

```shell
RUST_BACKTRACE=1 cargo run
```

### Reference

https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html
