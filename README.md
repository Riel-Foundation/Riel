# Riel: A Version Control System
Version control system inspired by Conflict-Free Resolution Data-Types and Logoot algorithm

State: Not ready for production yet

Alpha to-do list:
- [x] Initializing repositories
- [x] Binary executable file (Try something like: ```cargo build --out-dir ./binary -Z unstable-options```)
- [x] Adds, commits
- [X] Goto, rollback
- [ ] Author metadata
- [ ] Thrust (push), load (pull) from remotes
- [ ] CRDT algorithm implemented where possible

Manual installation for Linux:
1. Install Rust
2. Clone this repository
3. Create a folder for the generated binary file: ```mkdir myBinaryFolder```
3. Build the binary file: ```cargo build --release --out-dir ./myBinaryFolder -Z unstable-options```
4. Copy the binary file to your PATH: ```cp ./myBinaryFolder/riel /usr/local/bin```
Note: Updating the binary file is as simple as doing 3 and 4 again inside the repository's folder
