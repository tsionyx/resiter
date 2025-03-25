# resiter

A collection of helpful iterator adaptors for iterating over `Result<T, E>`.


## Examples

Here go some examples what you can do with the library.

* Altering T in `Iterator<Item = Result<T, E>>`

```rust
iter.map(|r| r.map(|a| alter(a))) // stdlib
iter.map_ok(|a| alter(a))         // resiter
```


* Altering T in `Iterator<Item = Result<T, E>>` with a function that might fail:

```rust
iter.map(|r| r.and_then(|a| alter(a))) // stdlib
iter.and_then_ok(|a| alter(a))         // resiter
```


* Unpacking T in `Iterator<Item = Result<Option<T>, E>>`

```rust
iter.map(|r| r.and_then(|o| o.ok_or_else(|| error()))) // stdlib
iter.inner_ok_or_else(|| error())                      // resiter
```


* Executing code for each error in `Iterator<Item = Result<T, E>>`

```rust
iter.map(|r| if let Err(e) = r { foo(); Err(e) } else { r })  // stdlib
iter.map(|r| r.map_err(|e| { foo(); e }))                     // stdlib
iter.on_err(|e| foo())                                        // resiter
```


* Fail fast the `Iterator<Item = Result<(), E>>`

```rust
for r in iter { r?; }                                         // stdlib
iter.fail_fast()?;                                            // resiter
```


* Process every item of `Iterator<Item = Result<Unit, E>>`, returning the last error only

```rust
fn erroneous<T, E>(x: T) -> Result<(), E> {
    ...
}

let err = iter.map(erroneous).filter_map(Result::err).last(); // stdlib
if let Some(last_err) = err { return Err(last_err); }         //

iter.map(erroneous).last_err()?;                              // resiter
```


## License

MPL 2.0

