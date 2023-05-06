slice
=====

A simple command-line tool that prints a slice from a file to stdout.


Building and installing
-----------------------

**Building:**

```
cargo build --release
```

**Installing:**

```
cargo install --path .
```

Usage
-----

```
slice my-file 0 10  # get the bytes 0 to 10 (exclusive)
slice my-file 0 0xA # hexadecimal can be used for the bounds
```

License
-------

_slice_ is distributed under the terms of the MIT license.
See `LICENSE` for details.
