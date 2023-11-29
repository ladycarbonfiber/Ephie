# Ephie -- An in memeory file system

In memory file system mostly as a playground for async rust

## Features
(all commands are in client emulations mirroring gnu tools for usability/discoverability)

- [X] Change working directory with `cd`

- [X] Create a new directory with `mkdir`

- [ ] Remove directory or "file" with `rm`

- [ ] Create a new file with `touch`

- [ ] Write to file with `cat`

- [ ] Read out file with `cat`

- [ ] Search working directory with `find`

- [ ] Support for path operations

  - [X] absolute path

  - [ ] parent path

  - [ ] relative path

  - [ ] auto mkdir


- [ ] Directory `cp` and `mv`

- [ ] AuthZ

  - [ ] Assign users and groups to directories/files

  - [ ] create new users/groups

  - [ ] assign/remove users to groups

  - [ ] switch users

  - [ ] Restrict access based on user/group membership 


## Requirements
### Build
- MAKE
- rust
- cargo

## Tests
make test

## Build
make all 

## Run
./ephied runs core daemon

./ephie-client -u <user> connect and interact with the file system

