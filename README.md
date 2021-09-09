Small JSON Parser in `no_std`
=============================

[![Build Status](https://travis-ci.org/rspencer01/microjson.svg?branch=master)](https://travis-ci.org/rspencer01/microjson)
[![Latest Version](https://img.shields.io/crates/v/microjson.svg)](https://crates.io/crates/microjson)
[![Coverage Status](https://coveralls.io/repos/github/rspencer01/microjson/badge.svg?branch=master)](https://coveralls.io/github/rspencer01/microjson?branch=master)

This library reads and parses JSON strings.

Its intended use case is to read a JSON payload once

It does _not_ serialise data.

Sample usage
------------

Simply put this in your `Cargo.toml`:
```toml
[dependencies]
microjson = { git = "https://github.com/rspencer01/microjson" }
```

You can read strings and integers easily:
```rust
use microjson::JSONValue;

let integer = JSONValue::parse("42")
    .expect("Could not parse json");

let value = integer.read_integer();

let string = JSONValue::parse("\"hello there\"")
    .expect("Could not parse json");

let value = integer.read_string();
```


You can read arrays like this:
```rust
use microjson::JSONValue;

let input = r#" [0, 1, 2, 3, 4, 5] "#;

let array = JSONValue::parse(input)
    .expect("Could not parse json");

for (n, item) in array.iter_array().unwrap().enumerate() {
    let value = item.read_integer()
                  .expect("Item was not an integer");
    assert_eq!(value, n as isize);
}
```

And, of course, any combination of the above:
```rust
use microjson::JSONValue;

let input = r#" { "arr": [3, "foo", 3.625, false] } "#;

let object = JSONValue::parse(input)
              .expect("Could not parse json");

assert_eq!(
    object.get_key_value("arr").unwrap().iter_array().unwrap().nth(2).unwrap().read_float(),
    Ok(3.625)
);
```

If you are unsure what kind of data you have, you can query the [`JSONValueType`].
```rust
use microjson::{JSONValue, JSONValueType};

let input = r#" 3.1415 "#;

let object = JSONValue::parse(input)
              .expect("Could not parse json");

match object.value_type {
    JSONValueType::String => {},
    JSONValueType::Number => {},
    JSONValueType::Object => {},
    JSONValueType::Array => {},
    JSONValueType::Bool => {},
    JSONValueType::Null => {},
}
```
