# Day06bの実装メモ

## qemuからテストできるようにする

### テストビルド

stdのテストは使えないため、カスタムテストフレームワークを使用します。  
カーネルのメインファイルに以下３行を追加しました。

```rust
// lib
#![feature(custom_test_frameworks)] // カスタムフレームワークの使用を宣言
#![test_runner(my_runner)] // テストランナーとなる関数名を指定
#![reexport_test_harness_main = "test_main"] // テストのエントリーポイントとなる関数名を指定
```

さらにランナーとテストケースを追加  
ちなにみにほぼ[The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/custom-test-frameworks.html)
から

```rust 

fn my_runner(tests: &[&i32]) {
    println!("test ");
    for t in tests {
        println!("test ");
        assert_eq!(0, **t);
        println!("success!");
    }
}

#[test_case]
const WILL_PASS: i32 = 0;

#[test_case]
const WILL_FAIL: i32 = 4;
```

cargo test時に実行させれないようにする、つまりtestフラグをtrueにした状態で
コンパイルだけできるようにするオプションがあるようなので、それを使ってみます。

```shell 
cargo test --no-run
```

しかし、以下のようなエラーが発生

```
language item required, but not found: `eh_personality`
```

パニック発生時に回復させるかなどの挙動を定義させるためのようですが、
workspaceのcargo.tomlにpanic=abortをすでに指定しているため、なぜこのエラーが出るのかわからない...

ターゲットを指定しているkernel.jsonに以下を追加し、cargo.tomlのpanic=abortを削除したところ
ビルドできるように！

```
"panic-strategy": "abort",
```

しかし、生成される出力先とバイナリ名が違うため、ここを対応する必要あり

### テストの実行

コンパイルされたファイルはtarget/kernel/debug/deps/にあるため、
以下のコマンドで検索できました。

```shell
find target/kernel/debug/deps/ -name *.elf
```

Makefileにテスト用のルールを追加してビルドスクリプトの対応完了！

## IOアドレス

## In/Out命令

[I/O instr](https://docs.oracle.com/cd/E19455-01/806-3773/6jct9o0aj/index.html)

## 参考文献

[The Rust Unstable Book](https://doc.rust-lang.org/beta/unstable-book/language-features/custom-test-frameworks.html)