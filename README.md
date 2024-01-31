# Riel: A Version Control System
Version control system inspired by Conflict-Free Resolution Data-Types and Logoot algorithm

State: Not ready for production yet
### Current features:
Adding, commiting, pseudo-random commit hashing, cloning using an https request via a tcp stream
### Alpha to-do list (Almost there):
- [ ] Config
- [ ] Remotes
- [ ] Thrust (push), load (pull) from remotes
### Beta to-do list:
- [ ] Author metadata
- [ ] Merging
- [ ] CRDT algorithm implemented where possible
- [ ] Branching
### Before release to-do list:
- [ ] Compression, deduplication, misc features...
- [ ] Documentation
- [ ] Tests
- [ ] Benchmarks
- [ ] Release


### Manual installation for Linux:
- Install Rust
- Clone this repository
- Run ```cargo build --release```
- Copy the binary file from ```target/release/riel``` to ```/usr/local/bin/```

### Repositories structure as a json:
```json
{
  "name": "repository-name",
  "children": [
    {
      "name": ".riel",
      "children": [{"Internal metadata is not clearly defined yet?" : "No, it isn't"}]
    },
    {
    "name": "folder1",
    "children": [
                  {
                    "name": "helloworld.rs",
                    "children": [],
                    "url": "https://myserver.local/user/repository-name/folder1/helloworld.rs"
                  }
                ],
    "url": null
    },
    {
    "name": "arootfile.txt",
    "children": [],
    "url": "https://myserver.local/user/repository-name/arootfile.txt"
    },
    {
      "name": ".rielignore",
      "children": [],
      "url": "https://myserver.local/user/repository-name/.rielignore"
    }
  ]
}
```

