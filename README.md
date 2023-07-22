# Polycubes

A [polycube](https://en.wikipedia.org/wiki/Polycube) generator inspired by Computerphile's recent [video](https://www.youtube.com/watch?v=g9n0a0644B4).

So far, only the 2D case has been implemented for simplicity, but extending to 3D shouldn't be too hard.

## Run

`-r` for release mode (optimized, way faster)

```
cargo run -r -- poly2d 15
```

## Print generated polys

Don't do this for large sizes. Many, many polys will be printed.

```
cargo run -r -- poly2d -r 4
```

## Help

```
cargo run -- --help
```

## TODO

- parallelize (massive gains to be had)
- canonicalize without trying all rotations (how?)
- reduce number of candidate polys that are tested
- eliminate hashset in favor of gigantic table indexed by the binary grid representation
