# MutStr
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

[MutStr](https://github.com/ThisAccountHasBeenSuspended/MutStr) is a good solution if you want to reduce memory consumption for e.g. hash tables like `<String, String>` and need something more efficient than `Box<str>` because you have to change the data at runtime.

- [MutStr](https://github.com/ThisAccountHasBeenSuspended/MutStr)
uses 16 bytes.
- [String](https://github.com/rust-lang/rust/blob/main/library/alloc/src/string.rs)
uses 24 bytes.

If you don't change the data at runtime, use `Box<str>`. If you prefer speed when adding new data, choose `String`. If you need low memory consumption and changeable data, choose `MutStr`.

## What does "... more efficient than `Box<str>` ..." mean?
The inner value of `Box<str>` cannot be changed, for this reason, the old `Box<str>` has to be replaced with a new allocated `Box<str>`, which is more inefficient than reallocate the inner value. [See our benchmarks](https://github.com/ThisAccountHasBeenSuspended/MutStr/blob/main/benches/all.rs)

## Benchmark results
This benchmark was updated at `January 13th, 2024` and can be found [here](https://github.com/ThisAccountHasBeenSuspended/MutStr/blob/master/benches/all.rs).
```
create_box                   28 ns/iter (+/- 3)
create_string                28 ns/iter (+/- 1)
create_mutstr                28 ns/iter (+/- 0)
replace_box_data             104 ns/iter (+/- 3)
replace_string_data          85 ns/iter (+/- 7)
replace_mutstr_data          82 ns/iter (+/- 1)
```

## Example
You can easily add new values or remove existing ones with `MutStr`.
```Rust
use mutstr::mutstr;

fn main() {
  let mut result = mutstr::from("hello");
  result += " my friend"; // Add -> " my friend"
  result -= " friend"; // Remove -> " friend"
  assert_eq!(result.as_str(), "hello my");
  
  result.push(" friend friend friend"); // Add -> " friend friend friend"
  result -= (2, " friend"); // Remove(2 times) -> " friend"
  assert_eq!(result.as_str(), "hello my friend");
  
  result += String::from(" :)"); // Add -> " :)"
  assert_eq!(result.as_str(), "hello my friend :)");
}
```

## Example
You can easily get a `&str` from a `MutStr` like you would with `String`.
```Rust
use mutstr::mutstr;

fn main() {
  let first = String::from("hello friend");
  let first_str = &*first; // String as `&str`
  let second = mutstr::from("hello friend");
  let second_str = &*second; // MutStr as `&str`
  assert_eq!(first_str, second_str);
}
```

## Example
You can easily use `MutStr` in hash tables and vectors, like you can with `Box<str>`.
```Rust
use std::collections::HashMap;
use mutstr::mutstr;

fn main() {
  let mut result = HashMap::<Box<str>, mutstr>::new();
  result.insert(Box::from("hello"), mutstr::from("friend"));
  
  let value = result.get_mut("hello").unwrap();
  *value += " :)";
  
  assert_eq!(value.as_str(), "friend :)");
}
```

[contributors-shield]: https://img.shields.io/github/contributors/ThisAccountHasBeenSuspended/MutStr.svg?style=for-the-badge
[contributors-url]: https://github.com/ThisAccountHasBeenSuspended/MutStr/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/ThisAccountHasBeenSuspended/MutStr.svg?style=for-the-badge
[forks-url]: https://github.com/ThisAccountHasBeenSuspended/MutStr/network/members
[stars-shield]: https://img.shields.io/github/stars/ThisAccountHasBeenSuspended/MutStr.svg?style=for-the-badge
[stars-url]: https://github.com/ThisAccountHasBeenSuspended/MutStr/stargazers
[issues-shield]: https://img.shields.io/github/issues/ThisAccountHasBeenSuspended/MutStr.svg?style=for-the-badge
[issues-url]: https://github.com/ThisAccountHasBeenSuspended/MutStr/issues
