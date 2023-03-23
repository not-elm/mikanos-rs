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

## イベントリングが反応しない不具合

イベントリングにイベントコマンドが渡された場合、デキューポインタの参照先に値がセットされます。

いろいろ調べた結果、XHC起動からイベントの受信だけなら以下の手順だけでできることが判明しました。
1. xHC Reset
2. Event Ring 登録
3. xHc Run

しかし、自分のコード上で動かしても全く受信されず...またいろいろ調べた結果
EventRingDequeuePointerに渡すアドレスは物理アドレスの必要がありますが、現状仮想アドレスになっていることが原因？

UEIF上のBootServiceが動いている間は恒等マップになっているようですが、Exit Boot Serviceで終了させた後はそのマッピングも破棄される？

取り合えずMikanOSのDay08の恒等マップの設定まで行ってみることにします。