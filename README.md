# sort_animals
sort animal JSON data from a file

## Assignment

We would like for you to parse the attached `data.json` file.

The goal is to turn each element in the JSON array into a type and instance in a language and place it in a heterogeneous array. Then demonstrate sorting that heterogeneous array on two separate properties, e.g., sort by weight or sort by color.

## Requirements

- [x] Pick any language you want, so long as it is a statically typed language.
- [x] Provide the steps necessary to build and execute your program.

## Build Steps

1. Install Rust and Cargo, instructions at: https://doc.rust-lang.org/cargo/getting-started/installation.html

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

2. In Terminal cd into sort_animals directory

```bash
$ cd ~/<path to directory where sort animals is>/sort_animals
```

3. To build run: 

```bash
$ cargo build --release
```

## Execution

**Debug Mode**

1. To run in debug mode cd to sort_animals directory 

```bash
$ cd ~/<path to directory where sort animals is>/sort_animals
```

2. type 

```bash 
cargo run -- [flags] <path of json file>
```

**Release Mode**

- build project

- run release version

```bash
$ ./target/release/sort_animals [flags] <path of json file>
```

## Sorting

**Sort by Color**

```bash
$ ./sort_animals -c data.json
```

**Sort by Weight**

```bash
$ ./sort_animals -w data.json
```

**Reverse Order Sort**

```bash
$ ./sort_animals -c -r data.json
```
