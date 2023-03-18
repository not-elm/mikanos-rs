# Day06cの実装メモ

## XHCIを動作させるまで

1. xhciを初期化
2. デバイスの最大接続数をMaxSlotsEnに設定

## DeviceContext

この構造体を要素とする配列を生成する必要がある(要素数はMaxSlotsEn + 1)

Device Context Base Address Array Pointer Register(DCBAAP)というレジスタにその配列の先頭アドレスをセットする。

デバイスコンテキストはSlotContext1つと、EndPointContext

### SlotContext

デバイスコンテキストの中の最初のフィールド
u32 * 8のサイズ

### EndPointContext

u32 * 8のサイズ