This program performs operations on a target directory specified by the user.
The available operations are:
- `tree`: Prints the directory tree of the target directory.
- `flatten`: Flatten the directory structure.d
- `duplicates`: Find duplicate files in the directory.
# Usage
```sh
cargo run <operation> <target_directory>
```
# Arguments
- `<operation>`: The operation to perform. Can be `tree`, `flatten`, or `duplicates`.
- `<target_directory>`: The path to the target directory.
