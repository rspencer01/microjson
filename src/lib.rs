// microjson - a no_std json parser in rust
// Copyright (C) 2021  Robert Spencer
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![doc = include_str!("../README.md")]
#![no_std]

/// Denotes the different types of values JSON objects can have
///
/// ### Numbers
/// Both floats and integers have a value type of [`JSONValueType::Number`].
///
/// ### Example
/// ```
/// # use microjson::*;
/// let json_value = JSONValue::parse("[1,2,3]").unwrap();
/// assert_eq!(json_value.value_type, JSONValueType::Array);
/// ```
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JSONValueType {
    String,
    Number,
    Object,
    Array,
    Bool,
    Null,
}

#[derive(Copy, Clone, Debug)]
pub struct JSONValue<'a> {
    contents: &'a str,
    pub value_type: JSONValueType,
}

fn trim_start(value: &str) -> (&str, usize) {
    let value_len = value.len();
    // NOTE(robert): This trims from the "start" which may be different for RTL languages.  What do
    // we do for JSON?
    let value = value.trim_start();
    (value, value_len - value.len())
}

impl<'a> JSONValue<'a> {
    pub fn parse(contents: &'a str) -> Result<JSONValue, &'static str> {
        let (contents, _) = trim_start(contents);
        let value_type = JSONValue::peek_value_type(contents)?;
        Ok(JSONValue {
            contents, value_type
        })
    }

    /// Guess the type of the JSON variable serialised in the input string
    ///
    /// This function will never give the _wrong_ type, though it may return a type even if the
    /// input string is not well formed.
    fn peek_value_type(contents: &'a str) -> Result<JSONValueType, &'static str> {
        // The contents must be trimmed
        match contents.chars().next() {
            Some('{') => {
                Ok(JSONValueType::Object)
            }
            Some('[') => {
                Ok(JSONValueType::Array)
            }
            Some('"') => {
                Ok(JSONValueType::String)
            }
            Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-') => {
                Ok(JSONValueType::Number)
            }
            Some('t' | 'f') => {
                Ok(JSONValueType::Bool)
            }
            Some('n') => {
                Ok(JSONValueType::Null)
            }
            _ => {
                return Err("Could not interpret start of token");
            }
        }
    }

    /// Confirm that this [`JSONValue`] is proper JSON
    ///
    /// This will scan through the entire JSON and confirm that it is properly formatted.
    /// See also [`JSONValue::parse_and_verify`]
    ///
    /// ## Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::parse("[1,{},\"foo\"]").unwrap();
    /// assert!(value.verify().is_ok());
    ///
    /// let value = JSONValue::parse("[,,{\"").unwrap(); // This will not error
    /// assert!(value.verify().is_err());
    /// ```
    pub fn verify(&self) -> Result<(), &'static str> {
        JSONValue::parse_with_len(self.contents)?;
        Ok(())
    }

    pub fn parse_and_verify(contents: &'a str) -> Result<JSONValue, &'static str> {
        let value = JSONValue::parse(contents)?;
        value.verify()?;
        Ok(value)
    }

    fn parse_with_len(contents: &'a str) -> Result<(JSONValue, usize), &'static str> {
        let (contents, whitespace_trimmed) = trim_start(contents);
        let (value_type, value_len) = match contents.chars().next() {
            Some('{') => {
                let mut value_len = 1;
                let mut contents = &contents[value_len..];
                while !contents.is_empty() {
                    if contents.trim_start().starts_with('}') {
                        value_len += trim_start(contents).1 + 1;
                        break;
                    }
                    let (item, item_len) = JSONValue::parse_with_len(contents)?;
                    if item.value_type != JSONValueType::String {
                        return Err("Cannot parse object key");
                    }
                    let (new_contents, whitespace) = trim_start(&contents[item_len..]);
                    contents = new_contents;
                    value_len += item_len + whitespace;
                    if contents.is_empty() {
                        return Err("End of stream while parsing object");
                    } else if contents.starts_with(':') {
                        value_len += 1;
                        contents = &contents[1..];
                    } else {
                        return Err("Illegal token while parsing object");
                    }

                    let (_, item_len) = JSONValue::parse_with_len(contents)?;
                    let (new_contents, whitespace) = trim_start(&contents[item_len..]);
                    contents = new_contents;
                    value_len += item_len + whitespace;
                    if contents.is_empty() {
                        return Err("End of stream while parsing object");
                    } else if contents.starts_with(',') {
                        value_len += 1;
                        contents = &contents[1..];
                    } else if !contents.starts_with('}') {
                        return Err("Illegal token while parsing object");
                    }
                }
                (JSONValueType::Object, value_len)
            }
            Some('[') => {
                let mut value_len = 1;
                let mut contents = &contents[value_len..];
                while !contents.is_empty() {
                    if contents.trim_start().starts_with(']') {
                        value_len += trim_start(contents).1 + 1;
                        break;
                    }
                    let (_, item_len) = JSONValue::parse_with_len(contents)?;
                    let (new_contents, whitespace) = trim_start(&contents[item_len..]);
                    contents = new_contents;
                    value_len += item_len + whitespace;
                    if contents.is_empty() {
                        return Err("End of stream while parsing array");
                    } else if contents.starts_with(',') {
                        value_len += 1;
                        contents = &contents[1..];
                    } else if !contents.starts_with(']') {
                        return Err("Illegal token while parsing array");
                    }
                }
                (JSONValueType::Array, value_len)
            }
            Some('"') => {
                let mut value_len = 1;
                let mut is_escaped = false;
                for chr in contents[1..].chars() {
                    value_len += chr.len_utf8();
                    if chr == '"' && !is_escaped {
                        break;
                    } else if chr == '\\' {
                        is_escaped = !is_escaped;
                    } else {
                        is_escaped = false;
                    }
                }
                (JSONValueType::String, value_len)
            }
            Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-') => {
                let mut value_len = 0;
                for chr in contents.chars() {
                    match chr {
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-' | 'e'
                        | 'E' | '.' => {
                            if chr == '-' && value_len > 0 {
                                return Err("Unexpected '-' while parsing number");
                            }
                            value_len += chr.len_utf8();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                (JSONValueType::Number, value_len)
            }
            Some('t') => {
                if &contents[..4] != "true" {
                    return Err("Unrecognised token");
                }
                (JSONValueType::Bool, 4)
            }
            Some('f') => {
                if &contents[..5] != "false" {
                    return Err("Unrecognised token");
                }
                (JSONValueType::Bool, 5)
            }
            Some('n') => {
                if &contents[..4] != "null" {
                    return Err("Unrecognised token");
                }
                (JSONValueType::Null, 4)
            }
            _ => {
                return Err("Could not interpret start of token");
            }
        };
        Ok((
            JSONValue {
                contents: &contents[..value_len],
                value_type,
            },
            whitespace_trimmed + value_len,
        ))
    }

    /// Reads the [`JSONValue`] as an integer
    ///
    /// If the type is not a [`JSONValueType::Number`], returns an `Err`.
    ///
    /// ### Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::parse("-24").unwrap();
    /// assert_eq!(value.read_integer(), Ok(-24));
    /// ```
    pub fn read_integer(&self) -> Result<isize, &'static str> {
        if self.value_type != JSONValueType::Number {
            return Err("Cannot parse value as integer");
        }
        let contents = self.contents.trim_end();
        str::parse(contents).or_else(|_| Err("Cannot parse as integer"))
    }

    /// Reads the [`JSONValue`] as a float
    ///
    /// If the type is not a [`JSONValueType::Number`], returns an `Err`.
    ///
    /// ### Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::parse("2.4").unwrap();
    /// assert_eq!(value.read_float(), Ok(2.4));
    /// ```
    pub fn read_float(&self) -> Result<f32, &'static str> {
        if self.value_type != JSONValueType::Number {
            return Err("Cannot parse value as float");
        }
        let contents = self.contents.trim_end();
        str::parse(contents).or_else(|_| Err("Cannot parse as float"))
    }

    /// Read the [`JSONValue`] as a string
    ///
    /// ## Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::parse("\"this is a string\"").unwrap();
    /// assert_eq!(value.read_string(), Ok("this is a string"));
    /// ```
    // TODO(robert): String can be escaped and all manner of trickery.  We need to deal with that
    // by returning some kind of iterator over characters here.
    pub fn read_string(&self) -> Result<&str, &'static str> {
        if self.value_type != JSONValueType::String {
            return Err("Cannot parse value as string");
        }
        Ok(&self.contents[1..self.contents.len() - 1])
    }

    /// Constructs an iterator over this array value
    ///
    /// If the value is not an [`JSONValueType::Array`], returns an error.
    pub fn iter_array(&self) -> Result<JSONArrayIterator<'a>, &'static str> {
        if self.value_type != JSONValueType::Array {
            return Err("Cannot parse value as an array");
        }
        Ok(JSONArrayIterator {
            contents: &self.contents[1..],
        })
    }

    // TODO(robert): This should be an iterator of `JSONValue`s
    pub fn get_key_value(&self, key: &str) -> Result<JSONValue, &'static str> {
        if self.value_type != JSONValueType::Object {
            return Err("Cannot parse value as an object");
        }
        let mut contents = &self.contents[1..];
        while !contents.is_empty() {
            let (this_key, key_len) = JSONValue::parse_with_len(contents).unwrap();
            contents = &contents[key_len..].trim_start()[1..];
            if this_key.read_string().unwrap() == key {
                return JSONValue::parse_with_len(contents).map(|x| x.0);
            } else {
                let (_, value_len) = JSONValue::parse_with_len(contents).unwrap();
                contents = &contents[value_len..].trim_start()[1..];
            }
        }
        Err("Key not found")
    }
}

/// An iterator through a JSON array value
pub struct JSONArrayIterator<'a> {
    contents: &'a str,
}

impl<'a> Iterator for JSONArrayIterator<'a> {
    type Item = JSONValue<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match JSONValue::parse_with_len(self.contents) {
            Ok((value, value_len)) => {
                self.contents = &self.contents[value_len..].trim_start()[1..];
                Some(value)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integer() {
        let (value, value_len) = JSONValue::parse_with_len("42").unwrap();
        assert_eq!(value.value_type, JSONValueType::Number);
        assert_eq!(value_len, 2);
        assert_eq!(value.read_integer(), Ok(42));
        assert!(value.read_string().is_err());

        assert_eq!(JSONValue::parse("-98").unwrap().read_integer(), Ok(-98));
        assert_eq!(JSONValue::parse("-99 ").unwrap().read_integer(), Ok(-99));
    }

    #[test]
    fn float() {
        let (value, value_len) = JSONValue::parse_with_len("3.141592").unwrap();
        assert_eq!(value.value_type, JSONValueType::Number);
        assert_eq!(value_len, "3.141592".len());
        assert!(value.read_integer().is_err());
        assert!(value.read_string().is_err());
        assert!((value.read_float().unwrap() - 3.141592).abs() < 0.0001);
    }

    #[test]
    fn string() {
        let (value, value_len) = JSONValue::parse_with_len("\"hello world\"").unwrap();
        assert_eq!(value.value_type, JSONValueType::String);
        assert_eq!(value_len, "\"hello world\"".len());
        assert!(value.read_integer().is_err());
        assert_eq!(value.read_string(), Ok("hello world"));
    }

    #[test]
    fn array() {
        let (value, value_len) = JSONValue::parse_with_len("[1,2,3]").unwrap();
        assert_eq!(value.value_type, JSONValueType::Array);
        assert_eq!(value_len, "[1,2,3]".len());
        let (value, value_len) = JSONValue::parse_with_len("[]").unwrap();
        assert_eq!(value.value_type, JSONValueType::Array);
        assert_eq!(value_len, "[]".len());
        let (value, value_len) = JSONValue::parse_with_len("  [\n  ]").unwrap();
        assert_eq!(value.value_type, JSONValueType::Array);
        assert_eq!(value_len, "  [\n  ]".len());
        let (value, value_len) = JSONValue::parse_with_len("[1  ,  2\t,\r3\n]").unwrap();
        assert_eq!(value.value_type, JSONValueType::Array);
        assert_eq!(value_len, "[1  ,  2\t,\r3\n]".len());

        assert!(value.read_integer().is_err());
        assert!(value.read_string().is_err());
        assert_eq!(
            value.iter_array().unwrap().nth(0).unwrap().read_integer(),
            Ok(1)
        );
        assert_eq!(
            value.iter_array().unwrap().nth(1).unwrap().read_integer(),
            Ok(2)
        );
        assert_eq!(
            value.iter_array().unwrap().nth(2).unwrap().read_integer(),
            Ok(3)
        );
    }

    #[test]
    fn object() {
        let input = "{
        \"id\": 0,
        \"name\": \"Ginger Fuller\"
      }";
        let (value, value_len) = JSONValue::parse_with_len(input).unwrap();
        assert_eq!(value.value_type, JSONValueType::Object);
        assert_eq!(value_len, input.len());

        assert!(value.read_integer().is_err());
        assert!(value.read_string().is_err());
        assert_eq!(value.get_key_value("id").unwrap().read_integer(), Ok(0));
        assert_eq!(
            value.get_key_value("name").unwrap().read_string(),
            Ok("Ginger Fuller")
        );

        assert!(JSONValue::parse("{\"foo\":[{}]}").is_ok());
        assert!(JSONValue::parse("[{\"foo\":{}}]").is_ok());
    }
    #[test]

    fn this_broke_once() {
        assert!(JSONValue::parse(
            r##"
[{"a":{"email":"d@"},"m":"#20\n\n.\n"}]
    "##
        )
        .is_ok());
    }

    #[test]
    fn integer_whitespace() {
        let (value, value_len) = JSONValue::parse_with_len("  42	").unwrap();
        assert_eq!(value.value_type, JSONValueType::Number);
        assert_eq!(value_len, "  42".len());
        let (value, value_len) = JSONValue::parse_with_len("\n 42\r").unwrap();
        assert_eq!(value.value_type, JSONValueType::Number);
        assert_eq!(value_len, "\n 42".len());
    }

    #[test]
    fn string_whitespace() {
        let (value, value_len) = JSONValue::parse_with_len("  \"foo me a bar\"	").unwrap();
        assert_eq!(value.value_type, JSONValueType::String);
        assert_eq!(value_len, "  \"foo me a bar\"".len());
        let (value, value_len) = JSONValue::parse_with_len("\n \"a bar\n I said.\"\r").unwrap();
        assert_eq!(value.value_type, JSONValueType::String);
        assert_eq!(value_len, "\n \"a bar\n I said.\"".len());
    }

    #[test]
    fn peeking_value_type() {
        assert_eq!(JSONValue::peek_value_type("123"), Ok(JSONValueType::Number));
        assert_eq!(JSONValue::peek_value_type("12.3"), Ok(JSONValueType::Number));
        assert_eq!(JSONValue::peek_value_type("12.3e10"), Ok(JSONValueType::Number));
        assert_eq!(JSONValue::peek_value_type("-3"), Ok(JSONValueType::Number));
        assert_eq!(JSONValue::peek_value_type("-3.5"), Ok(JSONValueType::Number));
        assert_eq!(JSONValue::peek_value_type("null"), Ok(JSONValueType::Null));
        assert_eq!(JSONValue::peek_value_type("true"), Ok(JSONValueType::Bool));
        assert_eq!(JSONValue::peek_value_type("false"), Ok(JSONValueType::Bool));
        assert_eq!(JSONValue::peek_value_type("[]"), Ok(JSONValueType::Array));
        assert_eq!(JSONValue::peek_value_type("[12]"), Ok(JSONValueType::Array));
        assert_eq!(JSONValue::peek_value_type("[1,2]"), Ok(JSONValueType::Array));
        assert_eq!(JSONValue::peek_value_type("[[]]"), Ok(JSONValueType::Array));
        assert_eq!(JSONValue::peek_value_type("\"foo\""), Ok(JSONValueType::String));
        assert_eq!(JSONValue::peek_value_type("{}"), Ok(JSONValueType::Object));
        assert_eq!(JSONValue::peek_value_type("{\"a\":2}"), Ok(JSONValueType::Object));
        assert!(JSONValue::peek_value_type("<").is_err());
        assert!(JSONValue::peek_value_type("bar").is_err());
    }

    #[test]
    fn verifying() {
        assert!(JSONValue::parse_and_verify(" 123 ").is_ok());
        assert!(JSONValue::parse_and_verify("[123]").is_ok());
        assert!(JSONValue::parse_and_verify("\"foo\"").is_ok());
    }
}
