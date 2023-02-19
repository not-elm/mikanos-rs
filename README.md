# MikanOS_rs

## Setup

setup.shを実行してください。

## Build and run

run.shを実行してください  
実行するとQemu Monitorが実行されます。  
下記コマンドで終了できます。

```qemu
qemu> q
```

## About Docker

開発環境構築用に、Dockerコンテナを起動し、コンテナ内にCargo Workspaceをボリュームするような方法を検討していましたが、
現在は使用していません。
一応Dockerfileとdocker-compose.ymlは残しています。

## MikanOsのビルドメモ

```shell

#edk2
source edksetup.sh
build

$HOME/osbook/devenv/qemu.sh $HOME/edk2/Build/MikanLoaderX64/DEBUG_CLANG38/X64/Loader.efi

# osbook/devenv

```

## Day02b

メモリーマップを取得し、mem_mapという名前のファイルに書き込むプログラムを書きます  
書き込んだファイルはdisk.imgをmntディレクトリにマウントし、そこから確認します。  
上記の確認する処理はqemu/cat_mem_map.shで宣言しています



