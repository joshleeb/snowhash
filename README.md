# Snowhash

Procedurally generate a unique snowflake from a hash.

There will eventually be a post on [joshleeb.com](https://joshleeb.com) explaining how the Snowhash algorithm works. In the mean time there is a demo up at [joshleeb.com/projects/snowhash](https://joshleeb.com/projects/snowhash).

## Building

To build just the `snowhash` library:

```bash
$ git clone https://github.com/joshleeb/snowhash.git && cd snowhash
$ cargo build
```

## Acknowledgement

This project is based on [Snowflake][snowflake] by Raph Levein which is "an implementation of procedural generation of snowflakes."

[snowflake]: http://levien.com/snowflake.html
