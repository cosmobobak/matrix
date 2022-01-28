# matrix

A rudimentary (but fast and correct!) crate for generic 2D matricies.

The matricies in this crate are dense, and backed by a Vec, making matricies fast, cache-friendly, and memory-efficient.

The matrix type also exposes pleasant iterator methods, allowing for immutable or mutable iteration over rows and columns, or the whole matrix.