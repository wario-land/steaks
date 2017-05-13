# Wario Land 4 ROM Hacking Docs

## About this document

This document contains information about the data structures and algorithms used by a game called Wario Land 4. The goal of this document is to aid my process developing *Steaks!*, a level editor for the game.

## Maps

### Overview

There are 242 maps and 92 tilesets in the game. Each map has multiple layers, including 3 foreground layers and multiple background ones. It seems like all layers share a single tileset.

### Map record table

Each map has a record in a global table located at `0x083F4E88` in the ROM. There are 242 records, they are 44 byte in size and contain information about the tileset, layers and game entities of the map. The background music of the maps are stored on a different location.

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u8        | Tileset index
| 0x01   | u8        | Foreground layer #1 settings
| 0x02   | u8        | Foreground layer #2 settings
| 0x03   | u8        | Foreground layer #3 settings
| 0x04   | u8        | (?) Background layer settings
| 0x05   | u24       | Padding
| 0x08   | u32_le    | Pointer to the fg #1 layer data
| 0x0C   | u32_le    | Pointer to the fg #2 layer data
| 0x10   | u32_le    | Pointer to the fg #3 layer data
| 0x14   | u32_le    | Pointer to the bg layer data
| 0x18   | u8        | (?) Layer priority
| 0x19   | u8        | (?) Layer priority
| 0x1A   | u8        | (?) Layer priority
| 0x1B   | u8        | (?) Layer priority (Always 0x00)
| 0x1C   | u32_le    | Hard game entity list pointer
| 0x20   | u32_le    | Normal game entity list pointer
| 0x24   | u32_le    | S-Hard game entity list pointer
| 0x28   | u8        | (?)
| 0x29   | u8        | (?)
| 0x2A   | u8        | (?)
| 0x2B   | u8        | (?)

*Question marks denote unsure information.*

### Layer settings

TODO:
0x00-Disabled
0x10-Enabled with collisions
0x11-Enabled without collision

### Map compression

TODO: RLE, not the BIOS RLE one

### Foreground layer data

TODO: w:u8, h:u8, rle_stream

### Background layer data

TODO: size:u8, rle_stream (1024/2048 bytes after decompression)

### Tileset indices

The game stores the tileset index of the current map at the RAM location 0x03000074. It uses it calculate the tileset record offset below.

| ID   | Description
| -----|------------
| 0x00 | Debug room
| 0x01 | Palm Tree Paradise
| 0x02 | Caves
| 0x03 | The Big Board
| 0x04 | The Big Board
| 0x05 | The Big Board?
| 0x06 | Wildflower Fields
| 0x07 | Toy Block Tower
| 0x08 | Factory
| 0x09 | Wildflower Underground?
| 0x0A | Wildflower/Jungle?
| 0x0B | Underwater?
| 0x0C | Toy Block Tower
| 0x0D | Toy Block Tower
| 0x0E | Toy Block Tower (purple)
| 0x0F | Doodle
| 0x10 | Dominoes
| 0x11 | Hall of Hieroglyphs
| 0x12 | Haunte House? (plus debug tiles)
| 0x13 | Crescent Moon Village outside
| 0x14 | Haunted House?
| 0x15 | Arabian outside
| 0x16 | Arabian inside
| 0x17 | Arabian
| 0x18 | Arabian
| 0x19 | Arabian
| 0x1A | Dominoes (blue)
| 0x1B | Dominoes (purple)
| 0x1C | Dominoes (teal)
| 0x1D | Factory
| 0x1E | Factory
| 0x1F | Jungle
| ...  | ...
| 0x30 | Unused in-game (Haunted House)
| 0x31 | Unused in-game (Haunted House)
| 0x32 | Unused in-game (Cardboard)
| ...  | ...
| 0x58 | Bonus room
| 0x59 | Bonus room
| 0x5A | Final level
| 0x5B | The Big Board end

### Tileset record table

Each tileset index indexes a record in a global tileset record table starting from `0x083F2298`. The record are 36 bytes long. There are 92 records in the table.

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u32       | Pointer to foreground graphics
| 0x04   | u32       | (?) Some data (migh be some data size)
| 0x08   | u32       | Pointer to palette
| 0x0C   | u32       | Pointer to background graphics
| 0x10   | u32       | (?) Some data (migh be some data size)
| 0x14   | u32       | (?) Pointer
| 0x18   | u32       | (?) Pointer
| 0x1C   | u32       | (?) Pointer
| 0x20   | u32       | (?) Pointer

TODO: List unused records.

### Entity lists

The game entities for each map and difficulty level stored in separate lists. The records are 3 bytes long and the first record is aligned to 32-bit. The end of the list marker is record with all of its fields containing 0xFF. The record structure is:

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u8        | Y-coordinate (top -> bottom)
| 0x01   | u8        | X-coordinate (left -> right)
| 0x02   | u8        | Entity index

### Entity indices

It seems like every map has a local selection of entities from a global list. Entities below 0x10 seems to be shared between every map.

| Index | Entity
| ----- | ------
| 0x00  | Game crash
| 0x01  | Box with top-right quadrant
| 0x02  | Box with bottom-right quadrant
| 0x03  | Box with bottom-left quadrant
| 0x04  | Box with top-left quadrant
| 0x05  | CD box
| 0x06  | Health box
| 0x07  | Large diamond
| 0x08  | Frog switch
| 0x09  | Keyzer
| 0x10  | Black cat
| 0x11  | Portal
| 0x12  | Walking torch
| 0x13  | Totsumen
| 0x14  | Rock to throw
| 0x15  | Opened box

## Text encoding
TODO: Import the text tables I reversed several years ago under my prev nickname.
### In-game

|        | x0 | x1 | x2 | x3 | x4 | x5 | x6 | x7 | x8 | x9 | xA | xB | xC | xD | xE | xF |
| ------ | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- | -- |
| **0x** | 0  | 1  | 2  | 3  | 4  | 5  | 6  | 7  | 8  | 9  | A  | B  | C  | D  | E  | F  |
| **1x** | G  | H  | I  | J  | K  | L  | M  | N  | O  | P  | Q  | R  | S  | T  | U  | V  |
| **2x** | W  | X  | Y  | Z  | a  | b  | c  | d  | e  | f  | g  | h  | i  | j  | k  | l  |
| **3x** | m  | n  | o  | p  | q  | r  | s  | t  | u  | v  | w  | x  | y  | z  | .  | &  |
| **4x** | あ | い | う | え | お | か | き | く | け | こ | さ | し | す | せ | そ | た |
| **5x** | ち | つ | て | と | な | に | ぬ | ね | の | は | ひ | ふ | へ | ほ | ま | み |
| **6x** | む | め | も | や | ゆ | よ | ら | り | る | れ | ろ | わ | を | ん | ぁ | ぃ |
| **7x** | ぅ | ぇ | ぉ | ゃ | ゅ | ょ | っ | が | ぎ | ぐ | げ | ご | ざ | じ | ず | ぜ |
| **8x** | ぞ | だ | ぢ | づ | で | ど | ば | び | ぶ | べ | ぼ | ぱ | ぴ | ぷ | ぺ | ぽ |
| **9x** | ア | イ | ウ | エ | オ | カ | キ | ク | ケ | コ | サ | シ | ス | セ | ソ | タ |
| **Ax** | チ | ツ | テ | ト | ナ | ニ | ヌ | ネ | ノ | ハ | ヒ | フ | ヘ | ホ | マ | ミ |
| **Bx** | ム | メ | モ | ヤ | ユ | ヨ | ラ | リ | ル | レ | ロ | ワ | ヲ | ン | ァ | ィ |
| **Cx** | ゥ | ェ | ォ | ャ | ュ | ョ | ッ | ガ | ギ | グ | ゲ | ゴ | ザ | ジ | ズ | ゼ |
| **Dx** | ゾ | ダ | ヂ | ヅ | デ | ド | バ | ビ | ブ | ベ | ボ | パ | ピ | プ | ペ | ポ |
| **Ex** | ヴ | '  | ,  | .  | -  | ~  | …  | !  | ?  | (  | )  | 「 | 」 | 『 | 』 | [  |
| **Fx** | ]  | ℃ | -  |    |    |    |    |    |    |    |    |    |    |    |    |    |

### Credits

# TODOs
* There are dozens of pointer-data pairs right after the map record table. They might be connected to the maps somehow.
* Passage destinations: 0x086391C4 + (i * 4), Values: 0x00-0x1C
* Another passage destination map: 0x08639068 + (i * 12), First byte: destination level index

- *xnagytibor*
