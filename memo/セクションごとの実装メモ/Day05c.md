# Day05cの実装メモ

## font.cppをrustで使う

ここでかなり時間を使いました...。  
build.rsを使うことで、ビルド前にcppのコンパイルができるとのことだったため、調査。  

その前にfont.cppを一部修正してget_fontを外部に公開するようにします。
```cpp
extern "C" {
    const uint8_t* get_font(char c);
}
```

その後build.rsを作成しましたが、 躓いた箇所が"cpp_link_stdlib(None)"の設定で、これを指定しないとlstdc++がないというエラーが出力されました。
```rust 
// kernel-lib/build.rs
cc::Build::new()
        .cpp(true)
        .static_flag(true)
        .object("hankaku.o")
        .cpp_link_stdlib(None)
        // .include("include/x86_64-linux-gnu/c++/11/bits/")
        .file("font.cpp")
        .compile("font");
```

ここまでできれば後はfont.rs内にget_fontを呼び出すコードを書くだけ！
```rust 

// kernel-lib/src/gop/font.rs
extern "C" {
     fn get_font(c: c_char) -> *mut u8;
}
```

...と思っていましたが、get_fontに渡す引数がc_charにしなくてはいけないという問題が残っていました！　　  

## rustのchar型からAscii文字に変換

rustのcharはUnicodeで構成されていますが、get_fontの引数になるためにAsciiの文字型に変換する必要があります。
charの[ドキュメント](https://doc.rust-lang.org/std/primitive.char.html)にいい感じにasciiに変換できそうなメソッドが記載されていたため、
それらを用いて変換する処理を追加してみました。

```rust

// font.rs

pub fn get_font_from(mut c: char) -> Option<*mut u8>{
    if !c.is_ascii(){
        return None;
    }

    if c.is_ascii_lowercase(){
        c.make_ascii_lowercase();
    }else{
        c.make_ascii_uppercase();
    }

    let char_ptr = unsafe{get_font(c as c_char)};
    if char_ptr == core::ptr::null_mut(){
        return None;
    }

    Some(char_ptr)
}
```

一応動いていますし、テストも通っているため問題はなさそう？  
ひとまずは解決ということにします。
```rust 
    /// 印字可能文字をすべて取得できるかのテスト
    /// Asciiの文字コード表はMikanOSの書籍の付録に記載されています。
   #[test]
    fn it_get_printable_ascii_codes() {
        let get_all_printable_ascii_codes = (0x20..=0x7Eu8).all(|code| get_font_from(char::from(code)).is_some());
        assert!(get_all_printable_ascii_codes);
    }

```