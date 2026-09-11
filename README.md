An easy-to-use tool to dump games from your PS4 written in Rust

WIP

A rough idea of the structrure

Phase 1
1. PS4 discovery
2. FTP connection
3. list filesystem
4. download one file
5. download directory recursively

Phase 2
1. detect currently running game
   1. Detect app
   2. Detect Patch
   3. Detect DLC
   4. Detect if its a remaster (i think remasters are titles like netflix, where the patch is embedded within the app. So there is no patch folder per se.)
2. dump base game
3. dump patch
4. dump DLC

Phase 3
1. progress / cancellation / resume
2. trophy handling
3. keystone
4. PFS / special PS4 behavior

Phase 4
1. egui GUI
2. Make tabs for "Titles", "Libraries", "Trophy"

Phase 5
1. Allow to dump FPKG from client -> PS4 (includes app, patch, DLC)