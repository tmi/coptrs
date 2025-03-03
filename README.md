
[![Static Badge](https://github.com/ecmwf/codex/raw/refs/heads/main/Project%20Maturity/emerging_badge.svg)](https://github.com/ecmwf/codex/raw/refs/heads/main/Project%20Maturity)

> \[!IMPORTANT\]
> This software is **Emerging** and subject to ECMWF's guidelines on [Software Maturity](https://github.com/ecmwf/codex/raw/refs/heads/main/Project%20Maturity).

# Combinatorial Optimization in Rust

This is an implementation of a few combinatorial optimization algorithms in Rust, with a Python interface.

Currently, we implement:
 * Floyd-Warshall -- minimum distance for all vertex pairs,
 * minimum nearest descendant for all vertex pairs (brute force).

All implementations first convert the python input into a rust representation, then release GIL and carry out the algorithm, and after reclaiming GIL construct a python object with the result and return it.

## Installation

```
uv pip install coptrs
```

## Development

```
uv pip install maturin
maturin develop . # installs into your current venv
maturin build -r . # builds a wheel
```

## Usage
There are usually multiple interfaces for each algorithm, differing by the object they take -- for example, Floyd-Warshall is exposed as `floyd_warshall` and `floyd_warshall_u32`, where the latter accepts graphs with integer vertex labels and the former accepts any vertex labels (and thus being a little slower as we need to do more preprocessing work).

For example usage, see the [floyd warshall benchmark](examples/benchm_floyd_warshall.py), which additionally compares performance against a native python implementation.

## Roadmap
* extend interface for more graph representation (list of neighbors, double dicts, ...)
* add more algorithms
* fix: make the release more convenient -- derive the Cargo.toml's version from the tag
* fix: have python api expose `__version__`, set some `__doc__`, etc
