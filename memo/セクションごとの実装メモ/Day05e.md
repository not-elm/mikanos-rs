# Day05eの実装メモ

## 230回改行処理をすると、左上の領域が描画されなくなる

Consoleを作ったのはいいのですが、興味本位で230回改行してみたら左上の領域が描画されなくなりました。
取り合えず一旦置いておくことにします...。

```rust
// kernel/lib

for _ in 0..230 {
console.write_str("Hello !Mikan Rust World!\n").unwrap();
}
```