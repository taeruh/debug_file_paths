# Minimal debugging example

In the `example` is the example directory structure. Run the binary in
`run/target/debug/run` in this directory or its subdirectory with no arguments or `.` as
argument. You'll see some output according to `run/src/main.rs` and the following changes
in the ignore crate:
```rust
// src/dir.rs; line 475

    Some(stripped_dot_slash) => stripped_dot_slash,
};
println!("dirpath: {dirpath:?}; path_prefix: {path_prefix:?}"); // CHANGED CHANGED CHANGED
println!("path before: {path:?}"); // CHANGED CHANGED CHANGED
let path = match strip_prefix(path_prefix, path) {
    None => abs_parent_path.join(path),
    Some(p) => {
        let p = match strip_prefix("/", p) {
            None => p,
            Some(p) => p,
        };
        abs_parent_path.join(p)
    },
};
println!("path after: {path:?}"); // CHANGED CHANGED CHANGED

for ig in self.parents().skip_while(|ig| !ig.0.is_absolute_parent) {
```
When running with `.`, one can see that the `.` prefix of the hidden files is removed from
them.
