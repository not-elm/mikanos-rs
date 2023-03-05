# Day05fの実装メモ

## エクスポートされたマクロのモジュールパスについて

print!をkernel-lib/src/gop/console.rs内に宣言したため、下記コードBの1.のように記述したら
モジュール解決できず...。  
いろいろ悩みましたが、カーネルのメインファイルないでマクロをインポートするときに 、
Aのようなパスになっていたため、2.のように記載したら通りました!

```rust
// A
// kernel/src/main.rs

use kernel_lib::{print, pritnln};
```
```rust
// B

#[macro_export]
macro_rules! println {
        () => {
            // 1. これは駄目
            $crate::gop::console::print!("\n");
            // 2. これが正解
            $crate::print!("\n");
        };
}
```

こんな罠があったとは...