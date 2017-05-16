# Wario Land 4 ROM Hacking Docs

## About this document

This document contains information about the data structures and algorithms used by a game called Wario Land 4. The goal of this document is to aid my process developing *Steaks!*, a level editor for the game. This document is work in progress, it's incomplete and some of its statements might be incorrect.

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
| 0x20 | Factory
| 0x21 | Junkyard
| 0x22 | Junkyard
| 0x23 | Pinball
| 0x24 | Pinball
| 0x25 | Pinball (with Gorilla)
| 0x26 | Jungle
| 0x27 | 40 Below Fridge?
| 0x28 | Jungle
| 0x29 | Jungle caves
| 0x2A | Hotel
| 0x2B | Hotel
| 0x2C | Hotel
| 0x2D | Hotel
| 0x2E | Hotel
| 0x2F | Hotel (outside)
| 0x30 | Unused in-game (Haunted House)
| 0x31 | Unused in-game (Haunted House)
| 0x32 | Unused in-game (Cardboard)
| 0x33 | Cardboard
| 0x34 | Caves
| 0x35 | Jungle
| 0x36 | Caves
| 0x37 | Lava level
| 0x38 | Caves
| 0x39 | Golden Passage
| 0x3A | Hotel
| 0x3B | Hotel
| 0x3C | Hotel
| 0x3D | Hotel
| 0x3E | 40 Below Fridge
| 0x3F | Factory
| 0x40 | Factory
| 0x41 | Arabian
| 0x42 | Arabian?
| 0x43 | Boss corridor
| 0x44 | Boss room? (golden)
| 0x45 | Frozen lava level
| 0x46 | Lava level
| 0x47 | Hall of Hieroglyphs
| 0x48 | Boss room?
| 0x49 | Boss room?
| 0x4A | Boss corridor
| 0x4B | Boss corridor
| 0x4C | Boss corridor
| 0x4D | Boss corridor
| 0x4E | Boss corridor
| 0x4F | Boss room?
| 0x50 | Hall of Hieroglyphs
| 0x51 | Jungle
| 0x52 | Wildflower
| 0x53 | Crescent Moon Village
| 0x54 | Crescent Moon Village
| 0x55 | Crescent Moon Village
| 0x56 | Toy Block Tower
| 0x57 | Pinball
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

## Music/SFX

The music and sound effects is stored in a MIDI-like event driven format. The music driver might be MP2000 from the official Nintendo SDK.

### Music/SFX table

Start from 0x08098028 in ROM, each record is 8 bytes long. 819 records. The records are not unique, there are duplication among them.

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u32       | Pointer to the music tracks record
| 0x04   | u16       | (?)
| 0x06   | u16       | (?) Not read by the game

### Music/SFX indices

| Index  | Description
| ------ | -----------
| 0x0000 | NUL
| 0x0016 | SFX: (?) Time's up
| 0x001C | NUL
| 0x001D | NUL
| 0x002F | NUL
| 0x0044 | NUL
| 0x0045 | NUL
| 0x005E | SFX: Ghost stealing coins
| 0x005F | SFX
| 0x0060 | SFX: (?) Something found
| 0x0080 | SFX
| 0x0118 | NUL
| 0x0119 | NUL
| 0x011A | NUL
| 0x011B | NUL
| 0x011C | NUL
| 0x012C | NUL
| 0x012D | NUL
| 0x012E | NUL
| 0x012F | NUL
| 0x0130 | NUL
| 0x0131 | NUL
| 0x0132 | NUL
| 0x0133 | NUL
| 0x0134 | NUL
| 0x0135 | SFX: (?) Something found
| 0x013D | NUL
| 0x0145 | NUL
| 0x0146 | NUL
| 0x0147 | NUL
| 0x0148 | NUL
| 0x0149 | NUL
| 0x014A | SFX: (?) Something found
| 0x0157 | NUL
| 0x0158 | NUL
| 0x0159 | NUL
| 0x015A | NUL
| 0x015B | NUL
| 0x015C | NUL
| 0x015D | NUL
| 0x0175 | NUL
| 0x0176 | NUL
| 0x0177 | NUL
| 0x0178 | NUL
| 0x0179 | NUL
| 0x017A | NUL
| 0x017B | NUL
| 0x017C | NUL
| 0x017D | NUL
| 0x017E | NUL
| 0x017F | NUL
| 0x0180 | NUL
| 0x0181 | NUL
| 0x0182 | NUL
| 0x0183 | NUL
| 0x0184 | NUL
| 0x0185 | NUL
| 0x0186 | NUL
| 0x0187 | NUL
| 0x0188 | NUL
| 0x0189 | NUL
| 0x018A | NUL
| 0x018B | NUL
| 0x018C | NUL
| 0x018D | NUL
| 0x018E | NUL
| 0x018F | NUL
| 0x0190 | NUL
| 0x0191 | NUL
| 0x0192 | NUL
| 0x0193 | NUL
| 0x0194 | NUL
| 0x0195 | NUL
| 0x0196 | NUL
| 0x0197 | NUL
| 0x0198 | NUL
| 0x0199 | NUL
| 0x019A | NUL
| 0x019B | NUL
| 0x019C | NUL
| 0x019D | NUL
| 0x019E | NUL
| 0x019F | NUL
| 0x01A0 | NUL
| 0x01A1 | NUL
| 0x01A2 | NUL
| 0x01A3 | NUL
| 0x01D4 | NUL
| 0x01D5 | NUL
| 0x01EE | NUL
| 0x01EF | NUL
| 0x01F0 | NUL
| 0x01F1 | NUL
| 0x01F2 | NUL
| 0x01F3 | NUL
| 0x0200 | Some jingle (Unused?)
| 0x0201 | Some jingle (Unused?)
| 0x0202 | (?)
| 0x0203 | SFX: Vheoh
| 0x0204 | SFX: Drang-drang-thung
| 0x0205 | (?)
| 0x0206 | (?)
| 0x0207 | SFX
| 0x0208 | SFX: Cymbals
| 0x0209 | SFX: Various percussions
| 0x020A | SFX: Barking and birds
| 0x020B | SFX: Flute-like tu-tih
| 0x020C | SFX: Some eastern instrument
| 0x020D | SFX: Cuckoo bird
| 0x020E | SFX: Frogs
| 0x020F | SFX: Medium pitched voice
| 0x0210 | SFX: High pitched voice
| 0x0211 | SFX: Blargh-blargh voice
| 0x0212 | SFX: High pitched voice
| 0x0213 | SFX: Baby-like blabbing
| 0x0214 | SFX: Hmm voice
| 0x0215 | SFX: Huh voice
| 0x0216 | SFX: Sheep
| 0x0217 | SFX: Police siren
| 0x0218 | SFX: Ting-tong
| 0x0219 | SFX: Bla-ha-hah
| 0x021A | SFX: Bubbling-tong
| 0x021B | NUL
| 0x021C | (?)
| 0x021D | (?)
| 0x021E | (?)
| 0x021F | (?)
| 0x0220 | (?)
| 0x0221 | (?)
| 0x0222 | (?)
| 0x0223 | SFX: Ti-tih
| 0x0224 | SFX: Some quiet sound
| 0x0225 | (?)
| 0x0226 | (?)
| 0x0227 | (?)
| 0x0228 | (?)
| 0x0229 | SFX: Some quiet sound
| 0x022A | SFX: Some quiet sound
| 0x022B | (?)
| 0x022C | SFX: Baseball minigame
| 0x022D | SFX: Baseball minigame
| 0x022E | SFX: Baseball minigame
| 0x022F | SFX: Baseball minigame
| 0x0230 | SFX: Baseball minigame
| 0x0231 | SFX: Baseball minigame
| 0x0232 | (?)
| 0x0233 | SFX: Baseball minigame
| 0x0234 | SFX: Baseball minigame
| 0x0235 | (?)
| 0x0236 | (?)
| 0x0237 | (?)
| 0x0238 | (?)
| 0x0239 | SFX: Wario Hop - Auh
| 0x023A | SFX: Vehicle sounds
| 0x023B | SFX: Some positive jingle
| 0x023C | SFX: Some negative jingle
| 0x023D | SFX: Wario Hop - Plomp
| 0x023E | SFX: Wario Hop - Watchit
| 0x023F | SFX: Wario Hop - Auh
| 0x0240 | SFX: Wario Hop - Yuhey
| 0x0241 | SFX: Wario Hop - Watchit
| 0x0242 | SFX: Wario Hop - Ding
| 0x0243 | SFX: Wario Hop - Auh
| 0x0244 | SFX: Minigame - Ding dong
| 0x0245 | SFX: Minigame - Doh doh
| 0x0246 | SFX: Minigame - Tadah
| 0x0247 | SFX: Minigame - Game over
| 0x0248 | (?)
| 0x0249 | (?)
| 0x024A | (?)
| 0x024B | (?)
| 0x024C | (?)
| 0x024D | (?)
| 0x024E | (?)
| 0x024F | SFX: Tick
| 0x0250 | (?)
| 0x0251 | SFX: Something obtained jingle
| 0x0252 | SFX: End of level jingle
| 0x0253 | NUL
| 0x0254 | NUL
| 0x0255 | NUL
| 0x0256 | NUL
| 0x0257 | NUL
| 0x0258 | NUL
| 0x0259 | NUL
| 0x025A | NUL
| 0x025B | NUL
| 0x025C | NUL
| 0x025D | NUL
| 0x025E | NUL
| 0x025F | NUL
| 0x0260 | NUL
| 0x0261 | NUL
| 0x0262 | NUL
| 0x0263 | NUL
| 0x0264 | NUL
| 0x0265 | NUL
| 0x0266 | NUL
| 0x0267 | NUL
| 0x0268 | NUL
| 0x0269 | BGM: Wario's workout
| 0x026A | BGM: Sound Room
| 0x026B | CD: About That Shepherd
| 0x026C | CD: Things That Never Change
| 0x026D | CD: Tomorrow's Blood Pressure
| 0x026E | CD: Beyond the Headrush
| 0x026F | CD: Driftwood & the Island Dog
| 0x0270 | CD: The Judge's Feet
| 0x0271 | CD: The Moon's Lamppost
| 0x0272 | CD: Soft Shell
| 0x0273 | CD: So Sleepy
| 0x0274 | CD: The Short Futon
| 0x0275 | CD: Avocado Song
| 0x0276 | CD: Mr. Fly
| 0x0277 | CD: Yesterday's Words
| 0x0278 | CD: The Errand
| 0x0279 | CD: You and Your Shoes
| 0x027A | CD: Mr. Ether & Planaria
| 0x027B | SFX: Intro
| 0x027C | BGM: Intro
| 0x027D | SFX: (?) Unused intro cinematic. What I hear: Wario halts his car, gets out and walks away.
| 0x027E | BGM: Some bossy music
| 0x027F | BGM: Overworld music
| 0x0280 | BGM: Some bossy music
| 0x0281 | NUL
| 0x0282 | BGM: Time warp
| 0x0283 | NUL
| 0x0284 | NUL
| 0x0285 | NUL
| 0x0286 | NUL
| 0x0287 | NUL
| 0x0288 | NUL
| 0x0289 | NUL
| 0x028A | NUL
| 0x028B | BGM: Palm Tree Paradise
| 0x028C | BGM: Palm Tree Paradise
| 0x028D | BGM: Palm Tree Paradise
| 0x028E | BGM: Wildflower Fields
| 0x028F | BGM: Mystic Lake
| 0x0290 | BGM: Mystic Lake
| 0x0291 | BGM: Mystic Lake
| 0x0292 | BGM: Monsoon Jungle
| 0x0293 | BGM: The Curious Factory
| 0x0294 | BGM: The Toxic Landfill
| 0x0295 | BGM: Pinball Zone
| 0x0296 | BGM: 40 Below Fridge
| 0x0297 | BGM: Toy Block Tower
| 0x0298 | BGM: The Big Board
| 0x0299 | BGM: Doodle Woods
| 0x029A | BGM: Domino Row
| 0x029B | BGM: Crescent Moon Village
| 0x029C | BGM: Arabian Night
| 0x029D | BGM: Hotel Horror
| 0x029E | BGM: Fiery Cavern
| 0x029F | BGM: Some bossy music (Unused?)
| 0x02A0 | BGM: Hall of Hieroglyps
| 0x02A1 | BGM: Bonus room
| 0x02A2 | BGM: Bonus room
| 0x02A3 | NUL
| 0x02A4 | NUL
| 0x02A5 | NUL
| 0x02A6 | NUL
| 0x02A7 | NUL
| 0x02A8 | NUL
| 0x02AA | BGM: Item Shop
| 0x02AB | BGM: Item's entrance to fight
| 0x02AC | BGM: Item's entrance to fight
| 0x02AD | (?)
| 0x02AE | (?)
| 0x02AF | BGM: Boss music
| 0x02B0 | BGM: Boss music
| 0x02B1 | BGM: Boss music
| 0x02B2 | NUL
| 0x02B3 | NUL
| 0x02B4 | NUL
| 0x02B5 | NUL
| 0x02B6 | NUL
| 0x02B7 | NUL
| 0x02B8 | NUL
| 0x02B9 | NUL
| 0x02BA | NUL
| 0x02BB | NUL
| 0x02BC | NUL
| 0x02BD | BGM: Mini-Game Shop
| 0x02BE | BGM: Wario Hop
| 0x02BF | BGM: Wario Hop
| 0x02C0 | BGM: Wario Hop
| 0x02C1 | BGM: Wario Hop
| 0x02C2 | BGM: Wario Hop
| 0x02C3 | BGM: Wario Hop (short)
| 0x02C4 | BGM: Wario's Roulette
| 0x02C5 | SFX: Baseball
| 0x02C6 | SFX: Baseball
| 0x02C7 | SFX: Game over
| 0x02C8 | SFX: Game over
| 0x02C9 | SFX: Baseball
| 0x02CA | SFX: Baseball
| 0x02CB | SFX: Baseball break
| 0x02CC | SFX: Baseball break
| 0x02CD | NUL
| 0x02CE | NUL
| 0x02CF | NUL
| 0x02D0 | NUL
| 0x02D1 | NUL
| 0x02D2 | NUL
| 0x02D3 | NUL
| 0x02D4 | NUL
| 0x02D5 | NUL
| 0x02D6 | NUL
| 0x02D7 | BGM: Palm Tree Paradise (without vocals)
| 0x02D8 | BGM: Palm Tree Paradise (without vocals)
| 0x02D9 | BGM: Palm Tree Paradise (only vocals)
| 0x02DA | Wario: Hurry up!
| 0x02DB | Wario: Hurry up!
| 0x02DC | Wario: Hurry up!
| 0x02DD | Wario: Hu-hu-hu-hurry up!
| 0x02DE | Wario: No-no-noh. Hurry up!
| 0x02DF | Wario: Ya-hoo!
| 0x02E0 | Wario: Yahoo!
| 0x02E1 | Wario: Yahoo!
| 0x02E2 | Wario: Yahoo!
| 0x02E3 | Wario: Ya-hoo!
| 0x02E4 | Wario
| 0x02E5 | Wario
| 0x02E6 | Wario
| 0x02E7 | Wario: Here we go!
| 0x02E8 | Wario: Here I go!
| 0x02E9 | Wario: Here I go!
| 0x02EA | Wario: Mamma mia.
| 0x02EB | Wario: Mamma mia.
| 0x02EC | Wario: Oh, boy.
| 0x02ED | Wario: Oh, boy.
| 0x02EE | Wario
| 0x02EF | Wario
| 0x02F0 | Wario: Sorry!
| 0x02F1 | Wario: So-so-so-sorry!
| 0x02F2 | Wario: Sorry!
| 0x02F3 | Wario
| 0x02F4 | Wario
| 0x02F5 | Wario: Go, bye, go, bye, go-bye-bye! (Unused?)
| 0x02F6 | Wario: Bye-bye-bye! (Unused?)
| 0x02F7 | Wario: Bye, bye! (Unused?)
| 0x02F8 | Wario: Höh.
| 0x02F9 | Wario: Hey, hey.
| 0x02FA | Wario: Ha, ha-ha!
| 0x02FB | Wario: Ha, ha-ha!
| 0x02FC | Wario: Ha-ha-ha-ha!
| 0x02FD | Wario: Hi-hi-hi-hi!
| 0x02FE | Wario: Hohey!
| 0x02FF | Wario: Hohey!
| 0x0300 | Wario: Hohey!
| 0x0301 | Wario: Yeah!
| 0x0302 | Wario
| 0x0303 | Wario
| 0x0304 | Wario
| 0x0305 | Wario: No-no!
| 0x0306 | Wario: Ergh, no-no!
| 0x0307 | Wario: No, no!
| 0x0308 | Wario: Wah-ah-ah-ah!
| 0x0309 | Wario: Wah-ah-ah-ah!
| 0x030A | (?)
| 0x030B | (?)
| 0x030C | (?)
| 0x030D | Wario: Eh-eh!
| 0x030E | Wario: Wah!
| 0x030F | Wario: Wow!
| 0x0310 | Wario: Wow!
| 0x0311 | Wario: Wow!
| 0x0312 | Wario: Yahoo!
| 0x0313 | NUL
| 0x0314 | NUL
| 0x0315 | NUL
| 0x0316 | NUL
| 0x0317 | NUL
| 0x0318 | NUL
| 0x0319 | NUL
| 0x031A | NUL
| 0x031B | NUL
| 0x031C | NUL
| 0x031D | NUL
| 0x031E | NUL
| 0x031F | NUL
| 0x0320 | BGM: Back to the portal
| 0x0321 | BGM: (?) (Unused?)
| 0x0322 | BGM: Princess cutscene
| 0x0323 | BGM: Japanese credits
| 0x0324 | BGM: Toy Block Tower
| 0x0325 | BGM: Monsoon Jungle
| 0x0326 | BGM: Japanese credits
| 0x0327 | BGM: Crescent Moon Village
| 0x0328 | BGM: Doodle Woods
| 0x0329 | BGM: Japanese credits
| 0x032A | BGM: Hall of Hieroglyphs (with vocals)
| 0x032B | BGM: Wildflower Fields
| 0x032C | BGM: Japanese credits
| 0x032D | BGM: Mystic Lake
| 0x032E | BGM: The Big Board
| 0x032F | BGM: Credits
| 0x0330 | BGM: Credits
| 0x0331 | BGM: Credits
| 0x0332 | BGM: Credits



Note about getting music indices:
* Breakpoint at 0x08001DD2 and checking the r0 register
* Or setting an on-write breakpoint at 0x03003202

### Music/SFX tracks

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u32       | Track count
| 0x04   | u32       | Track #1 data pointer
| 0x08   | u32       | Track #2 data pointer
| ...    | ...       | ...

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
* Passage destinations: 0x086391C4 + (i * 4), Values: 0x00-0x1C
* Another passage destination map: 0x08639068 + (i * 12), First byte: destination level index
* Overworld passage connections: 0x086392D0 + (i * 4), values starting from 0x00: Entry, Emerald, Ruby, Topaz, Sapphire, Pyramid, Sound Room,
* Starting from 0x083F7828 there are (u32, gfx pointer) pairs. Have no idea what they do.
* Collectible appearances (?) -> 16 x u16 per map, for HoH screen 3 starts at 0x083F8978
* Starting from 0x083B14F0 there are 32 bytes long records. Connected somehow to the entities.


- *xnagytibor*
