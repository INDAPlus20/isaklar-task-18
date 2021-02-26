# isaklar-task-18

To create index-file: 
```rust
fn main() {
let path = args().nth(1).unwrap();

    // This is for generating index file
    generate_compact_index(path);

    // This is for generating magic file
    //generate_magic_file(path);

    // These are for lookup
    // let input = WINDOWS_1252.encode(&path).0;
    // find(&input[0..]);
    }
```
Then run with the token file-path as argument.

To create magic-file:
```rust
fn main() {
    let path = args().nth(1).unwrap();

    // This is for generating index file
    //generate_compact_index(path);

    // This is for generating magic file
    generate_magic_file(path);

    // These are for lookup
    // let input = WINDOWS_1252.encode(&path).0;
    // find(&input[0..]);
}
```
Then run with index file-path as argument

To lookup a word:
```rust
fn main() {
    let path = args().nth(1).unwrap();

    // This is for generating index file
    //generate_compact_index(path);

    // This is for generating magic file
    // generate_magic_file(path);

    // These are for lookup
    let input = WINDOWS_1252.encode(&path).0;
    find(&input[0..]);
}
```
Then run with the word as an argument.
