# Demo recordings

The game stores the index of the currently played demo recording at the location 0x03000020.

| Index | Demo level
| ----- | ----------
| 0x00  | Hall of Hieroglyphs
| 0x01  | Palm Tree Paradise
| 0x02  | Wildflower Fields
| 0x03  | Mystic Lake
| 0x04  | Monsoon Jungle
| 0x05  | Cractus
| 0x06  | The Curious Factory
| 0x07  | 40 Below Fridge
| 0x08  | Pinball Zone
| 0x09  | Aerodent
| 0x0A  | Toy Block Tower
| 0x0B  | Doodle Woods
| 0x0C  | Domino Row
| 0x0D  | Arabian Night
| 0x0E  | Fiery Cavern
| 0x0F  | Hotel Horror

## Pointer tables

These tables are indexed by the index above.

| Offset     | Count  | Description
| ---------- | ------ | -----------
| 0x0878F5F4 | 0x10   | Button states
| 0x0878F634 | 0x10   | Timings

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
- Where does the game define the stages, areas and starting coordinates of the demos?
