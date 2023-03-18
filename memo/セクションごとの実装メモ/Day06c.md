# Day06cの実装メモ

## XHCIを動作させるまで

1. xhciを初期化
2. デバイスの最大接続数をMaxSlotsEnに設定
3. デバイスコンテキストの配列を生成
4. 配列の先頭アドレスをDCBAAPに書き込む
5. CommandRingの生成

## メモリの書き込み幅

OperationalRegisterは32Bits幅で読まないと正しく設定されない。

## DeviceContext

この構造体を要素とする配列を生成する必要がある(要素数はMaxSlotsEn + 1)

Device Context Base Address Array Pointer Register(DCBAAP)というレジスタにその配列の先頭アドレスをセットする。

デバイスコンテキストはSlotContext1つと、EndPointContext
1024Byte

### SlotContext

デバイスコンテキストの中の最初のフィールド
32Byteのサイズ

### EndPointContext

32Byte * 31のサイズ

## Command Ring Control Register(CRCR)

Command Ring Control Register (CRCR)
Address: Operational Base + (18h)
Default Value: 0000 0000 0000 0000h
Attribute: RW
Size: 64 bits
The Command Ring Control Register provides Command Ring control and status
capabilities, and identifies the address and Cycle bit state of the Command Ring
Dequeue Pointer.