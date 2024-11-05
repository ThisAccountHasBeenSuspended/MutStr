# MutStr
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]

`MutStr` is a mutable alternative for `&str`.

- `&str`
- `MutStr`
- - uses 16 bytes.
- `String`
- - uses 24 bytes.

`MutStr` was written as a replacement for `Box<str>` and `String` for hashtables. If you don't change the data at runtime, use `Box<str>`. If you prefer speed when adding new data, choose `String`. If you need low memory consumption and changeable data, choose `MutStr`.

> [!TIP]
> `MutStr` is compatible with [`serde`](https://crates.io/crates/serde).
> 
> Use `features` as in the following example to be able to use `serde`:<br>
> `mutstr = { version = "0.3.0", features = ["serde"] }`

## Examples

### Assign
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

### `MutStr` as `&str`
Get a `&str` from a `MutStr`.

```Rust
use mutstr::mutstr;

fn main() {
  let (first, second) = (
    "hello friend",
    mutstr::from("hello friend")
  );
  assert_eq!(first, second.as_str());
}
```

### Hashtables and Vectors
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
