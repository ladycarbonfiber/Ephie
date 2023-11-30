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

- [X] Write to file with `write`

- [X] Read out file with `read`

- [X] Search working directory with `find`

- [ ] `cp`

  - [X] Files

  - [ ] Directories

- [ ] `mv`

  - [ ] Files

  - [ ] Directories

- [ ] AuthZ

  - [ ] Assign users and groups to directories/files

  - [ ] create new

    - [ ] users

    - [ ] groups 

  - [ ] assign/remove users to groups

  - [ ] switch users

  - [ ] Restrict access based on user/group membership 

- [X] CI
  - [X] Build on PR
  - [X] Test on PR

## Current Scope Limitiations

- Relative parent directory (ie `../..` or `foo/../bar`)

- Client file writes are overwrite

- Only support unix; windows paths are awful


## Requirements
### Build
- MAKE
- rust (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`) to install the tool chain comes with cargo
- cargo

## Tests
make test

## Build
make all

## Release
make release

## Run
make rund

make client

