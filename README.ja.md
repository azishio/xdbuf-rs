[English](README.md) | 日本語

# xdbuf

再利用可能な多次元バッファを提供します。
次元が同じであれば任意のサイズで再初期化でき、メモリの再割り当て回数を最小限に抑えます。

また、配列を探索するための`Walker`構造体を提供します。

## 使い方

```rust
use xdbuf::{XDBuf, Walker, step2d};

fn main() -> Result<(), anyhow::Error> {
    // インスタンスの生成
    let mut buf = XDBuf::new([5, 6], 0).unwrap();
    assert_eq!(buf.get(0), Some(&0));
    assert_eq!(buf.len(), 30);

    // 初期化配列から再初期化
    // [1, 2, 3,
    //  4, 5, 6,
    //  7, 8, 9]
    let initial_vec = (1..=9).collect::<Vec<_>>();
    buf.init_with_vec([3, 3], initial_vec).unwrap();
    assert_eq!(buf.get(0), Some(&1));
    assert_eq!(buf.get(8), Some(&9));

    // 配列を探索する`Walker`を生成
    let mut walker = buf.walker_from_m([1, 1])?;
    assert_eq!(buf.get(walker.index_s()), Some(&5));

    //下に移動
    assert_eq!(step2d::DOWN, [0, -1]);
    walker.as_(&step2d::DOWN)?;
    assert_eq!(buf.get(walker.index_s()), Some(&2));

    // 値が8以上になるまで移動
    walker.as_until(|&value, _index| { value >= 8 })?;
    assert_eq!(buf.get(walker.index_s()), Some(&8));

    // 値の書き換え
    buf.set(walker.index_s(), 100)?;

    Ok(())
}
```

## ライセンス (License)

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](../vec-x-rs/LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](../vec-x-rs/LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

(The English in the README and documentation comments has been translated into English by DeepL and ChatGPT.)
