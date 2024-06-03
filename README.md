## Merkle Tree root calculator

This program takes a file, containing a number of hashes, as an input, producing
the Merkle Tree root as output.

Assumptions made for the input file:

- A hash is a 64 bytes long ASCII string
- A hash is a base16 string
- A hash is a lowercase string
- A hash algorithm is `sha256`
- Hashes are separated by newlines ('\n')

In order to calculate the hash of the node (which contains a pair of hashes),
values of the containing hashes are concatenated.

If the node contains only a left hash (if the number of hashes in the file is
not equal to the power of two), its value is reused as the right hash.

#### Build

```bash
cargo build -r
```

#### Usage

```
Usage: merkle_root [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>  Input file, containing one base16 sha256 hash per line
  -m, --mode <MODE>  Calculation mode (default: depth-walk) [possible values: depth-walk, width-walk]
  -h, --help         Print help (see more with '--help')
  -V, --version      Print version

```

#### Algorithms

All algorithms will be O(n\*log(n)) time complexity, because there is no other
way to calculate the root hash except to traverse all the paired hashes from the
very bottom of the tree, until calculation converges into a single hash. The
only difference between algorithms is space complexity and the possibility to
utilize parallel calculations.

1. Depth-walk

Time complexity: O(n\*log(n)), sequential only.

Space complexity: O(log(n)).

The idea of this algorithm is to traverse the Merkle tree depth-first, starting
from the left and recursively calculating the right branch until the list of
hashes ends. This allows reading the hashes from the file one by one, reducing
the memory footprint.

Usage: `target/release/merkle_root -f input.txt -m depth-walk`

1. Width-walk

Time complexity: O(n\*log(n)), parallel.

Space complexity: O(n).

The idea of this algorithm is to read all the hashes into memory and calculate
the Merkle tree layer by layer, until it converges into a single hash. This
opens possibilities to utilize parallel calculations, which speeds up the
program, requiring much more memory.

Usage: `target/release/merkle_root -f input.txt -m width-walk`

#### Tests

```
cargo test
```

#### Microbenches

NB: all microbenchmarks were done on an Apple MacBook Pro M1 with 8 active
cores. Absolute numbers are of no interest here, the only interesting thing is
how results differ depending on the algorithm used.

1. Time benches

```
cargo bench
```

The depth-walk algorithm performs calculations in ~6.5 ms. The width-walk
algorithm performs calculations in ~2 ms, 3.5x times faster.

1. Memory benches

I didn't find any simple way to perform memory benches with the criterion crate,
so I used the time utility to find some numbers.

Example:

```
/usr/bin/time -l target/release/merkle_root -f input.txt -m width-walk
```

In order to find out how much memory the program consumes on itself, I used the
`input_minimal.txt` as a list of hashes. This file contains only one hash, and
both algorithms just return the single hash without any calculations. With both
algorithms, the peak memory footprint was the same, `1016576` bytes. I will
refer to this number as the base footprint.

Running the depth-walk algorithm on `input.txt` resulted in `1114944` bytes of
peak memory footprint, which corresponds to `98368` bytes of overhead. The
width-walk algorithm on the same input resulted in `3163264` (average) bytes of
peak memory footprint, which corresponds to `2146688` bytes overhead, which is
approximately 22 times more.

#### Alternative algoritms

1. In-place calculation

It is possible to perform Merkle Tree root calculation in-place (within a file,
modifying and truncating it). This will have the same time complexity as any
other algorithm, but will have outstanding O(1) space complexity. Despite time
complexity being the same, overall performance will be poor due to a large
number of disk operations.

2. GPU calculation

Modern GPUs can perform thousands of hash computations at the same time, giving
the time complexity of amortized O(n\*log(n)), giving a very significant
performance boost.
