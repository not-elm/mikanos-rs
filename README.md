# MikanOS-rs

## 環境構築

setup.shを実行してください。

## ブートローダーの実行方法

bootloaderディレクトリ直下でcargo runを実行することでqemuが立ち上がります。  
実行されたshell上でQemuMonitorが実行されます。  
下記コマンドで終了できます。

```qemu
qemu> q
```

## Dockerfileについて

開発環境構築用に、Dockerコンテナを起動し、コンテナ内で開発する方法を検討していましたが、
Clionでは VscodeのDevContainerのような拡張機能が見つからなかっため、使用していません。  
一応Dockerfileとdocker-compose.ymlは残しています。

## MikanOsのビルドメモ

この項目はMikanOSのビルド方法をメモしただけなので、MikanOS-rsとは関係ありません。

```shell
#edk2
source edksetup.sh
build

$HOME/osbook/devenv/qemu.sh $HOME/edk2/Build/MikanLoaderX64/DEBUG_CLANG38/X64/Loader.efi
```

## Day02a

Qemu上でHelloWorldをする!




