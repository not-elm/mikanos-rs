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

