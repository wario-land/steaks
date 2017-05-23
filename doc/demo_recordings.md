# Demo recordings

## Pointer tables

| Offset     | Count  | Description
| ---------- | ------ | -----------
| 0x0878F5F4 | 0x10   | Button states
| 0x0878F634 | 0x10   | Timings

| Offset | Data type | Demo level
| ------ | --------- | ----------
| 0x00   | u32_ptr   | Hall of Hieroglyphs
| 0x04   | u32_ptr   | Palm Tree Paradise
| 0x08   | u32_ptr   | Wildflower Fields
| 0x0C   | u32_ptr   | Mystic Lake
| 0x10   | u32_ptr   | Monsoon Jungle
| 0x14   | u32_ptr   | Cractus
| 0x18   | u32_ptr   | The Curious Factory
| 0x1C   | u32_ptr   | 40 Below Fridge
| 0x20   | u32_ptr   | Pinball Zone
| 0x24   | u32_ptr   | Aerodent
| 0x28   | u32_ptr   | Toy Block Tower
| 0x2C   | u32_ptr   | Doodle Woods
| 0x30   | u32_ptr   | Domino Row
| 0x34   | u32_ptr   | Arabian Night
| 0x38   | u32_ptr   | Fiery Cavern
| 0x3C   | u32_ptr   | Hotel Horror

## Data format and playback

The button states and timings are both defined as u16 arrays. The button state data copied to 0x03002CC8, the timings are copied to 0x03002EC8 for playback. The timings might be given in frames.

| Value  | Button
| -------| ------
| 0x0001 | A
| 0x0002 | B
| 0x0004 | (?)
| 0x0008 | (?)
| 0x0010 | Right
| 0x0020 | Left
| 0x0040 | Up
| 0x0080 | Down
| 0x0100 | (?)
| ...    | ...

## TODO
* Where does the game define the stages, areas and starting coordinates of the demos?
