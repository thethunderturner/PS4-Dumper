An easy-to-use tool to dump games from your PS4 written in Rust

WIP

A rough idea of the structure

Phase 1
1. [x] PS4 discovery
2. [x] FTP connection
3. [x] list filesystem
4. [x] download one file
5. [x] download directory recursively

Phase 2
1. detect currently running game
   1. [x] Detect app
   2. [x] Detect Patch
   3. [ ] Detect DLC
   4. [x] Parse param.sfo
2. [x] dump base app
3. [x] dump patch
4. [ ] dump DLC
5. [ ] After the program selects the PS4, check if its jailbroken. If it isn't, display a message and quit.

Phase 3
1. egui GUI
2. Make tabs for "Titles", "Libraries", "Trophy"

Phase 4
1. Allow to dump FPKG from client -> PS4 (includes app, patch, DLC)

Phase 5
1. Allow to dump libraries
2. Allow to dump fonts
3. Allow to dump trophy, and to decrypt the trophy