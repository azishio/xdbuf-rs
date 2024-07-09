English | [日本語](README.ja.md)

# xdbuf

Provides a reusable multidimensional buffer.
It can be reinitialized to any size as long as the dimensions are the same, minimizing the number of memory
reallocations.

Also provides a `Walker` structure to traverse the array.

## Usage

```rust
use xdbuf::{XDBuf, Walker, step2d};

fn main() -> Result<(), anyhow::Error> {
    // Create an instance
    let mut buf = XDBuf::new([5, 6], 0).unwrap();
    assert_eq!(buf.get(0), Some(&0));
    assert_eq!(buf.len(), 30);

    // Reinitialize from an initial array
    // [1, 2, 3,
    //  4, 5, 6,
    //  7, 8, 9]
    let initial_vec = (1..=9).collect::<Vec<_>>();
    buf.init_with_vec([3, 3], initial_vec).unwrap();
    assert_eq!(buf.get(0), Some(&1));
    assert_eq!(buf.get(8), Some(&9));

    // Generate a `Walker` to traverse the array
    let mut walker = buf.walker_from([1, 1])?;
    assert_eq!(buf.get(walker.index()), Some(&5));

    // Move down
    assert_eq!(step2d::DOWN, [0, -1]);
    walker.as_(&step2d::DOWN)?;
    assert_eq!(buf.get(walker.index()), Some(&2));

    // Move until the value is greater than or equal to 8
    walker.as_until(|&value, _index| { value >= 8 })?;
    assert_eq!(buf.get(walker.index()), Some(&8));

    // Rewrite the value
    buf.set(walker.index(), 100)?;

    Ok(())
}
```

## License

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](../vec-x-rs/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](../vec-x-rs/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

(The English in the README and documentation comments has been translated into English by DeepL and ChatGPT.)
