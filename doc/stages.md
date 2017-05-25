# Stages

## Stage header indices

- Starts at: 0x086391C4
- Record count: 36
- Record type: u32

| Index | Offset     | Description
| ----- | ---------- | -----------
| 0x00  | 0x086391C4 | Hall of Hieroglyphs
| 0x01  | 0x086391C8 | *Null*
| 0x02  | 0x086391CC | Debug Stage
| 0x03  | 0x086391D0 | *Null*
| 0x04  | 0x086391D4 | Spoiled Rotten
| 0x05  | 0x086391D8 | Mini-Game Shop
| 0x06  | 0x086391DC | Palm Tree Paradise
| 0x07  | 0x086391E0 | Wildflower Fields
| 0x08  | 0x086391E4 | Mystic Lake
| 0x09  | 0x086391E8 | Monsoon Jungle
| 0x0A  | 0x086391EC | Cractus
| 0x0B  | 0x086391F0 | Mini-Game Shop
| 0x0C  | 0x086391F4 | The Curious Factory
| 0x0D  | 0x086391F8 | The Toxic Landfill
| 0x0E  | 0x086391FC | Pinball Zone
| 0x0F  | 0x08639200 | 40 Below Fridge
| 0x10  | 0x08639204 | Cuckoo Condor
| 0x11  | 0x08639208 | Mini-Game Shop
| 0x12  | 0x0863920C | Toy Block Tower
| 0x13  | 0x08639210 | The Big Board
| 0x14  | 0x08639214 | Doodle Woods
| 0x15  | 0x08639218 | Domino Row
| 0x16  | 0x0863921C | Aerodent
| 0x17  | 0x08639220 | Mini-Game Shop
| 0x18  | 0x08639224 | Crescent Moon Village
| 0x19  | 0x08639228 | Arabian Night
| 0x1A  | 0x0863922C | Fiery Cavern
| 0x1B  | 0x08639230 | Hotel Horror
| 0x1C  | 0x08639234 | Catbat
| 0x1D  | 0x08639238 | Mini-Game Shop
| 0x1E  | 0x0863923C | Golden Passage
| 0x1F  | 0x08639240 | *Null*
| 0x20  | 0x08639244 | *Null*
| 0x21  | 0x08639248 | *Null*
| 0x22  | 0x0863924C | Golden Diva
| 0x23  | 0x08639250 | Mini-Game Shop

## Stage headers

- Starts at: 0x08639068
- Record count: 29
- Record length: 12 bytes

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u8        | Area header pointer index
| 0x01   | u8        | Area count
| 0x02   | u8        | Always 0x0A
| 0x03   | u24       | Hard mode time limit
| 0x06   | u24       | Normal mode time limit
| 0x09   | u24       | S-Hard mode time limit

| Index | Offset     | Description
| ----- | ---------- | -----------
| 0x00  | 0x08639068 | Hall of Hieroglyphs
| 0x01  | 0x08639074 | Palm Tree Paradise
| 0x02  | 0x08639080 | Wildflower Fields
| 0x03  | 0x0863908C | Mystic Lake
| 0x04  | 0x08639098 | Monsoon Jungle
| 0x05  | 0x086390A4 | Cractus
| 0x06  | 0x086390B0 | Mini-Game Shop
| 0x07  | 0x086390BC | The Curious Factory
| 0x08  | 0x086390C8 | The Toxic Landfill
| 0x09  | 0x086390D4 | Pinball Zone
| 0x0A  | 0x086390E0 | 40 Below Fridge
| 0x0B  | 0x086390EC | Cuckoo Condor
| 0x0C  | 0x086390F8 | Mini-Game Shop
| 0x0D  | 0x08639104 | Toy Block Tower
| 0x0E  | 0x08639110 | The Big Board
| 0x0F  | 0x0863911C | Doodle Woods
| 0x10  | 0x08639128 | Domino Row
| 0x11  | 0x08639134 | Aerodent
| 0x12  | 0x08639140 | Mini-Game Shop
| 0x13  | 0x0863914C | Crescent Moon Village
| 0x14  | 0x08639158 | Arabian Night
| 0x15  | 0x08639164 | Fiery Cavern
| 0x16  | 0x08639170 | Hotel Horror
| 0x17  | 0x0863917C | Catbat
| 0x18  | 0x08639188 | Mini-Game Shop
| 0x19  | 0x08639194 | Golden Diva
| 0x1A  | 0x086391A0 | Spoiled Rotten
| 0x1B  | 0x086391AC | Debug Stage
| 0x1C  | 0x086391B8 | Golden Passage

## Area header pointers

- Starts at: 0x0878F280
- Record count: 25
- Record type: u32_ptr

| Index | Offset     | Description
| ----- | ---------- | -----------
| 0x00  | 0x0878F280 | Hall of Hieroglyphs
| 0x01  | 0x0878F284 | Palm Tree Paradise
| 0x02  | 0x0878F288 | Wildflower Fields
| 0x03  | 0x0878F28C | Mystic Lake
| 0x04  | 0x0878F290 | Monsoon Jungle
| 0x05  | 0x0878F294 | The Curious Factory
| 0x06  | 0x0878F298 | The Toxic Landfill
| 0x07  | 0x0878F29C | Pinball Zone
| 0x08  | 0x0878F2A0 | 40 Below Fridge
| 0x09  | 0x0878F2A4 | Toy Block Tower
| 0x0A  | 0x0878F2A8 | The Big Board
| 0x0B  | 0x0878F2AC | Doodle Woods
| 0x0C  | 0x0878F2B0 | Domino Row
| 0x0D  | 0x0878F2B4 | Crescent Moon Village
| 0x0E  | 0x0878F2B8 | Arabian Night
| 0x0F  | 0x0878F2BC | Hotel Horror
| 0x10  | 0x0878F2C0 | Fiery Cavern
| 0x11  | 0x0878F2C4 | Cractus
| 0x12  | 0x0878F2C8 | Cuckoo Condor
| 0x13  | 0x0878F2CC | Aerodent
| 0x14  | 0x0878F2D0 | Catbat
| 0x15  | 0x0878F2D4 | Golden Diva
| 0x16  | 0x0878F2D8 | Spoiled Rotten
| 0x17  | 0x0878F2DC | Golden Passage
| 0x18  | 0x0878F2E0 | Debug Stage

### Area headers

- Starts at: 0x083F4E88
- Record count: 242
- Record length: 44 bytes

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u8        | Tileset index
| 0x01   | u8        | Foreground layer #1 settings
| 0x02   | u8        | Foreground layer #2 settings
| 0x03   | u8        | Foreground layer #3 settings
| 0x04   | u8        | (?) Background layer settings
| 0x05   | u24       | Padding
| 0x08   | u32_ptr   | Pointer to the fg #1 layer data
| 0x0C   | u32_ptr   | Pointer to the fg #2 layer data
| 0x10   | u32_ptr   | Pointer to the fg #3 layer data
| 0x14   | u32_ptr   | Pointer to the bg layer data
| 0x18   | u8        | (?) Layer priority
| 0x19   | u8        | (?) Layer priority
| 0x1A   | u8        | (?) Layer priority
| 0x1B   | u8        | (?) Layer priority (Always 0x00)
| 0x1C   | u32_ptr   | Hard mode entity list pointer
| 0x20   | u32_ptr   | Normal mode entity list pointer
| 0x24   | u32_ptr   | S-Hard mode entity list pointer
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

### Pointers for map entrance lists

They start from 0x0878F21C and point to map entrance lists.

| Offset | Data type | Stage
| ------ | --------- | -----
| 0x00   | u32_ptr   | Hall of Hieroglyphs
| 0x04   | u32_ptr   | Palm Tree Paradise
| 0x08   | u32_ptr   | Wildflower Fields
| 0x0C   | u32_ptr   | Mystic Lake
| 0x10   | u32_ptr   | Monsoon Jungle
| 0x14   | u32_ptr   | The Curious Factory
| 0x18   | u32_ptr   | The Toxic Landfill
| 0x1C   | u32_ptr   | (?) Pinball Zone
| 0x20   | u32_ptr   | 40 Below Fridge
| 0x24   | u32_ptr   | Toy Block Tower
| 0x28   | u32_ptr   | The Big Board
| 0x2C   | u32_ptr   | Doodle Woods
| 0x30   | u32_ptr   | Domino Row
| 0x34   | u32_ptr   | Crescent Moon Village
| 0x38   | u32_ptr   | Arabian Night
| 0x3C   | u32_ptr   | Hotel Horror
| 0x40   | u32_ptr   | Fiery Cavern
| 0x44   | u32_ptr   | (?) Emerald Passage Boss Corridor
| 0x48   | u32_ptr   | (?) Ruby Passage Boss Corridor
| 0x4C   | u32_ptr   | (?) Topaz Passage Boss Corridor
| 0x50   | u32_ptr   | (?) Sapphire Passage Boss Corridor
| 0x54   | u32_ptr   | (?) Golden Passage Boss Corridor
| 0x58   | u32_ptr   | (?) Entry Passage Boss Corridor
| 0x5C   | u32_ptr   | (?) Golden Passage

### Map entrance list

The data from 0x083F2F88 in ROM, each record is 12 bytes long. Records filled with 0x00 separates stages. 660 records.

Note: from 0x0878E780 to 0x0878F970 there are several of pointers pointing to these records.

| Offset | Data type | Description
| ------ | --------- | -----------
| 0x00   | u8        | (?) Possible values: 0x01, 0x02, 0x03, (0x04, 0x05)
| 0x01   | u8        | (?) Looks like a sequence/map number
| 0x02   | u8        | (?) Might be the X-coordinate
| 0x03   | u8        | (?) Might be the X-coordinate
| 0x04   | u8        | (?) Might be the Y-coordinate
| 0x05   | u8        | (?) Might be the Y-coordinate
| 0x06   | u8        | Destination (0x00 - Back to the overworld)
| 0x07   | u8        | (?) Possible values: 0x00, 0x08, 0x18, 0x20, 0x24, 0x40, 0xE0, ...
| 0x08   | u8        | (?) Similar to the previous value
| 0x09   | u8        | (?) Connected somehow to entities
| 0x0A   | u16       | Music index, 0x0000 means no change on entering
