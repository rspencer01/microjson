Small JSON Parser in `no_std`
=============================

[![Latest Version](https://img.shields.io/crates/v/microjson.svg)](https://crates.io/crates/microjson)
[![Test and lint](https://github.com/rspencer01/microjson/actions/workflows/test.yml/badge.svg)](https://github.com/rspencer01/microjson/actions/workflows/test.yml)
[![Coverage Status](https://coveralls.io/repos/github/rspencer01/microjson/badge.svg?branch=master)](https://coveralls.io/github/rspencer01/microjson?branch=master)
[![Docs](https://img.shields.io/docsrs/microjson)](https://docs.rs/microjson/latest/microjson/)

This library reads and parses JSON strings.

Its intended use case is to read a JSON payload once.

It does _not_ serialise data.

Sample usage
------------

Simply put this in your `Cargo.toml`:
```toml
[dependencies]
microjson = "0.1"
```

You can read strings and integers easily:
```rust
# use microjson::{JSONValue, JSONParsingError};
# fn main() -> Result<(), JSONParsingError> {
let integer = JSONValue::parse("42")?;

let value : isize = integer.read_integer()?;

let string = JSONValue::parse("\"hello there\"")?;

let value : &str = string.read_string()?;
# Ok(())
# }
```

You can read arrays like this:
```rust
# use microjson::{JSONValue, JSONParsingError};
# fn main() -> Result<(), JSONParsingError> {
let input = r#" [0, 1, 2, 3, 4, 5] "#;

let array = JSONValue::parse(input)?;

for (n, item) in array.iter_array()?.enumerate() {
    let value = item.read_integer()?;
    assert_eq!(value, n as isize);
}
# Ok(())
# }
```

And, of course, any combination of the above:
```rust
# use microjson::{JSONValue, JSONParsingError};
# fn main() -> Result<(), JSONParsingError> {
let input = r#" { "arr": [3, "foo", 3.625, false] } "#;

let object = JSONValue::parse(input)?;

assert_eq!(
    object.get_key_value("arr")?.iter_array()?.nth(2).unwrap().read_float()?,
    3.625
);
# Ok(())
# }
```

If you are unsure what kind of data you have, you can query the [`JSONValueType`].
```rust
# use microjson::{JSONValue, JSONValueType, JSONParsingError};
# fn main() -> Result<(), JSONParsingError> {
let input = r#" 3.1415 "#;

let object = JSONValue::parse(input)?;

match object.value_type {
    JSONValueType::String => {},
    JSONValueType::Number => {},
    JSONValueType::Object => {},
    JSONValueType::Array => {},
    JSONValueType::Bool => {},
    JSONValueType::Null => {},
}
# Ok(())
# }
```

Verifying Data
--------------

To load some JSON, you need only call
```rust
# use microjson::JSONValue;
let value = JSONValue::parse(r#" [1,2,3,5"foo"] "#);
```
However, this data is malformed.  [`JSONValue::parse`] will return an `Ok` result, as to determine that the data was corrupt would require scanning through the entire string.
The error would only be reported when you attempted to iterate to the fourth item and parse it as a value.

If you need to know that the data is sound, use [`JSONValue::verify`].  Alternatively, you can parse and verify in one step.
```rust
# use microjson::JSONValue;
let value = JSONValue::parse_and_verify(r#" [1,2,3,5"foo"] "#);
```

Features
--------
  * [x] All JSON types
  * [x] Strings with escape sequences
  * [x] Parse ints (using [built in parser](https://doc.rust-lang.org/1.56.0/std/primitive.isize.html#method.from_str_radix))
  * [x] Parse floats (using [built in parser](https://doc.rust-lang.org/1.56.0/std/primitive.f32.html#method.from_str))
  * [x] Iterators over arrays
  * [x] Object key lookup
  * [x] Iterators over objects
  * [x] Verify JSON
