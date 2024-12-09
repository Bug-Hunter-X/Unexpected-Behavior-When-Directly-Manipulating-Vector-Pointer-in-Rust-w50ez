# Unexpected Behavior When Directly Manipulating Vector Pointer in Rust

This repository demonstrates a potential issue in Rust where directly manipulating a vector's underlying pointer using `as_mut_ptr()` can lead to unexpected behavior if not handled carefully. The primary concern is that operations modifying the vector (such as adding or removing elements) can invalidate the pointer, causing data corruption or crashes.

The `bug.rs` file contains code that showcases this issue.  The solution, demonstrated in `bugSolution.rs`, emphasizes safer alternatives like indexing or iterators instead of direct pointer manipulation.