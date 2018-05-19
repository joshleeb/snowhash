# Snowhash

Procedurally generate a unique snowflake from a hash.

There will eventually be a post on [joshleeb.com][https://joshleeb.com] explaining how the Snowhash algorithm works and how I compiled it to WASM and eventually show up in the blog post.

## Usage

```bash
$ tree . -L 1
.
├── snowhash       // library containing procedural generation logic
├── snowhash_png   // binary to generate snowflake as png image
└── snowhash_wasm  // wasm library to be used with javascript
```

### snowhash_png

```bash
$ cargo build && ./target/debug/snowhash_png --help
Snowhash 0.1.0
Josh Leeb-du Toit <josh.leebdutoit@gmail.com>
Procedurally generate a unique snowflake from a hash

USAGE:
    snowhash_png [OPTIONS] <HASH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output <OUTPUT>    Output path for the generated PNG

ARGS:
    <HASH>    Hash to use for generating the snowflake
```

For example, using the first commit hash from this repo: `1f83a1a1cdfa28cb1eb42a41078be4080051e87d`

```bash
$ snowhash_png 1f83a1a1cdfa28cb1eb42a41078be4080051e87d
```

Which will generate `1f83a1a1cdfa28cb1eb42a41078be4080051e87d.png`

TODO: Add image.

## Building

### snowhash

To build just the `snowhash` library:

```bash
$ git clone https://github.com/joshleeb/snowhash.git && cd snowhash/snowhash
$ cargo build
```

### snowhash_png

To build the `snowhash_png` binary:

```bash
$ git clone https://github.com/joshleeb/snowhash.git && cd snowhash
$ cargo build
```

And then to run:

```bash
$ ./target/debug/snowhash_png
```

#### Dependencies

`snowhash_png` uses [`cairo`](https://www.cairographics.org) to create the PNG of the snowflake. The `cairo-rs` crate is a wrapper around the `cairo` clib so it must be installed. 

### snowhash_wasm

To build the `snowhash_wasm` library:

```bash
$ git clone https://github.com/joshleeb/snowhash.git && cd snowhash/snowhash_wasm
$ ./build.sh
```

The `build.sh` script will compile the snwohash library to target `wasm32-unknown-unknown`. To run a development server use `webpack-dev-server` and navigate to `localhost:8080` in your browser. Or, to create a minified version, just run `webpack` which will create the `dist/` directory containing the minified `js` and `wasm` code.

### Dependencies

`webpack` is required to bundle the compiled `wasm` and the `js` scripts. And `webpack-dev-server` is required to serve the scripts in a development environment. 

## Acknowledgement

This project is based on [Snowflake][snowflake] by Raph Levein which is "an implementation of procedural generation of snowflakes."

[snowflake]: http://levien.com/snowflake.html
