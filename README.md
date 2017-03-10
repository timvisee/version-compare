# version-compare (Rust library)
An easy to use library for rust to compare version strings.

This library is still a work in progress.

## Usage
This shows a possible usage representation of the library. This might change, as the library is still a work in progress.

```Rust
let a = "1.2";
let b = "1.5.1";

// Version string parsing
let a_ver = Version::from(a);
let b_ver = Version::from(b);

// Comparisons
a_ver.compare(b_ver); // Returns: CompOp::LT
b_ver.compare(a_ver); // Returns: CompOp::GT
VersionCompare::compare(a_ver, b_ver); // Returns: CompOp::LT

// Boolean tests
assert!(a_ver.is_compare(b_ver, CompOp::LT));
assert!(b_ver.is_compare(a_ver, CompOp::GT));
assert!(VersionCompare::is_compare(a_ver, b_ver, CompOp::LT));

// Match
match a_ver.compare(b_ver) {
    CompOp::LT => println!("Version a is less than b"),
    CompOp::EQ => println!("Version a is equal to b"),
    CompOp::GE => println!("Version a is greater than b")
}
```

## License
This project is released under the GNU GPL-3.0 license. Check out the [LICENSE](LICENSE) file for more information.