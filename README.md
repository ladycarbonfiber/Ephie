# Ephie -- An in memeory file system

In memory file system mostly as a playground for async rust

## Features
(When possible commands mirror gnu utils for discoverability)

- [ ] Terminal Client

  - [X] Interactive

  - [ ] Multi-user Concurrent Support

  - [ ] Auto clear

  - [ ] Help

- [ ] List with `ls`

  - [X] Working Directory

  - [ ] absolute path

  - [ ] relative path

  - [ ] parent path

- [X] Change working directory with `cd`

  - [X] absolute path

  - [X] parent path

  - [X] relative path

- [X] Create a new directory with `mkdir`

  - [X] absolute path

  - [X] parent path

  - [X] relative path

  - [X] auto mkdir

- [X] Remove directory or "file" with `rm`

  - [X] absolute path

  - [X] parent path

  - [X] relative path

- [X] Create a new file with `touch`

- [ ] Write to file with `write`

- [ ] Read out file with `read`

- [ ] Search working directory with `find`

- [ ] `cp`

  - [ ] Files

  - [ ] Directories

- [ ] `mv`

  - [ ] Files

  - [ ] Directories

- [ ] AuthZ

  - [ ] Assign users and groups to directories/files

  - [ ] create new users/groups

  - [ ] assign/remove users to groups

  - [ ] switch users

  - [ ] Restrict access based on user/group membership 

- [X] CI
  - [X] Build on PR
  - [X] Test on PR

## Current Scope Limitiations

- Relative parent directory (ie `../..` or `foo/../bar`)

- Client file writes are overwrite and don't support spaces


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

