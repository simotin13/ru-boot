# GR-Peach

## Spec
 - CPU:  RZ/A1H(R7S721001VCBG), Cortex-A9
 - RAM: 内蔵10MB
 - ROM: 外部FLASH 8MB

## LED
 - P6_12 PIN_LED_USER(赤)
 - P6_13 PIN_LED_RED
 - P6_14 PIN_LED_GREEN
 - P6_15 PIN_LED_BLUE
P6_13～P6_15は同一LED

P6_0:SW(IN)
P6_1:NC
P6_2:TGT_TXD
P6_3:TGT_RXD
P6_4:XSCK
P6_5:XSSL
P6_6:XMOSI
P6_7:XMISO
P6_8:XBDIO1

# Address Map
0x2090_0000～0x209F_FFFF: 内蔵RAM ページ4
...
0x2000_0000～0x2001_FFFF: 内蔵RAM ページ0
0x1C00_0000～0x1FFF_FFFF: SPIマルチI/Oバス領域 チャネル1 (64MB)
0x1800_0000～0x1BFF_FFFF: SPIマルチI/Oバス領域 チャネル0 (64MB)
0x1400_0000～0x17FF_FFFF: CS5空間 (64MB)
0x1000_0000～0x13FF_FFFF: CS4空間 (64MB)
0x0C00_0000～0x0FFF_FFFF: CS3空間 (64MB)
0x0800_0000～0x0BFF_FFFF: CS2空間 (64MB)
0x0400_0000～0x07FF_FFFF: CS1空間 (64MB)
0x0000_0000～0x03FF_FFFF: CS0空間 (64MB)

# SDCard
SPI2
 - P8_4: CD_DAT3 → SPI2 SSL(Slave Select)
 - P8_5: CMD     → SPI2 MOSI
 - P8_3: CLK     → SPI2 SCK
 - P8_6: DAT0    → SPI2 MISO
 - P7_8: C_DET   → IRQ IRQ1
 - Pullup: DAT1
 - Pullup: DAT2

受信: Master Input Slave Output
送信: Master Output Slave Input

★RZ SPIドライバ
8421

# Clock
CPU Clock XIin SG-8003CGPCB: 13.3333MHz →EXTAL
USB Clock USB X1 48MHz →EXTAL

PLLは30倍固定。
IΦ: CPU Clock 400MHz
GΦ: 画像処理 Clock 200MHz
BΦ: 内部バス Clock 100MHz
P1Φ: 周辺 Clock 1 Ma 66.67MHz
P0Φ: 周辺 Clock 0 Ma 33.33MHz
CKIO: 外部バス Cock 100MHz

// 制御レジスタ(SPCR)
// 7: SPRIE  受信割り込み許可 ON
// 6: SPE    SPI有効 ON
// 5: SPTIE  送信割り込み許可 ON
// 4: SPEIE  エラー割り込み許可 ON
// 3: MSTR   マスタモード(0: スレーブモード 1: マスタモード) マスターモード
// 2: MODFEN モードフォルト有効 ON
// 1: 0固定
// 0: 0固定
RSPI2->SPCR = 


## link

# Boot Mode 
ブートモード:3 SPI マルチI/O バス空間に接続されたシリアルフラッシュメモリからブート
MD_BOOT0: HI固定
MD_BOOT1: LOW固定
MD_BOOT2: HI固定

