# Day03aの実装メモ

## kernel.elfのエントリーポイントが呼び出されない

QemuモニタからRIPレジスタの値を確認したところ、ベースアドレスとはかけ離れたアドレスがセットされている！  
オリジナルMikanOSのブートローダーからも同様に呼び出せない...。

```shell
info registers
``` 

kernel.jsonに"exe-suffix": ".elf"というプロパティを追記したところ、オリジナルMikanOSのブートローダーからは呼び出せた！