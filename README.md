# MikanOS-rs

uchanさんによって執筆された[ゼロからの OS 自作入門](http://zero.osdev.jp/)の中で製作するMikanOSをRustで挑戦するプロジェクトです。

進捗や実装時のメモは[Zenn](https://zenn.dev/elm)に挙げています。

## 環境構築

setup.shを実行してください。

## ブートローダーの実行方法

makeコマンドでビルドと実行ができます。

```shell
make
```

make debugでGDBを使用したデバッグ実行ができます。

```shell
make debug
```

Clionの場合、実行設定でRemoteDebugを追加し、下記項目の通り設定することでデバッグ実行の設定ができます。

- Debugger
    - GBDのbinaryがあるディレクトリのパス(WSL上で実行する場合、WSL内のGDBを指定する必要があります。)
- Target remote args
    - tcp::1234
- Symbol file
    - kernel.elfへのパス

終了する際はQemuモニタ上で下記コマンドを使用してください。

```qemu
qemu> q
```

## Test

テストは下記コマンドで実行できます。

```shell
make test
```

テストビルドの際はカーネルのターゲットディレクトリをクリーンする必要があるため、少々時間がかかります。

## Dockerfileについて

開発環境構築用に、Dockerコンテナを起動し、コンテナ内で開発する方法を検討していましたが、
Clionでは VscodeのDevContainerのような拡張機能が見つからなかっため、使用していません。  
一応Dockerfileとdocker-compose.ymlは残しています。

## MikanOSのビルドメモ

この項目はMikanOSのビルド方法をメモしただけなので、MikanOS-rsとは関係ありません。

```shell
#edk2
source edksetup.sh
build

$HOME/osbook/devenv/qemu.sh $HOME/edk2/Build/MikanLoaderX64/DEBUG_CLANG38/X64/Loader.efi
```

## Day02a

Qemu上でHelloWorldをする!

## Day02b

メモリーマップの情報をファイルに出力する！

## Day03a

カーネルファイルをロードして、エントリーポイントを呼び出す！

## Day03b

GraphicOutputProtocolを使い、ブートローダーからピクセルを描画する！

## Day03c

カーネルからピクセルを描画する！

## Day04c

ピクセルライターを作成する！

## Day05a

'A'という文字を描画する！

## Day05c

フォントを列挙したオブジェクトファイルから文字を描画する！

## Day05e

コンソール制御をする機構の作成をする！

## Day05f

println!マクロの実装をする！

## Day06

マウスドライバを実装する！

## Day07

マウスを割り込みに対応させる！

## Day08

メモリ管理を実装する！

## Day09

重ね合わせ処理を実装する！

## Day10

簡易的なウィンドウを作成する!

## Day11

Apicタイマの割り込み処理を実装する!

## Day12

キーボード入力を実装する!

## Day13a

コンテキストスイッチを手動で出来るようにする！

### Day13b

タイマ割り込み内でコンテキストスイッチを出来るようにする！

## Day13d

タスクを3つ以上生成できるようにする!
