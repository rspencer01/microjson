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

mod error;
pub use error::JSONParsingError;

/// Denotes the different types of values JSON objects can have
///
/// ### Numbers
/// Both floats and integers have a value type of [`JSONValueType::Number`].
///
/// ### Example
/// ```
/// # use microjson::*;
/// let json_value = JSONValue::load("[1,2,3]");
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
    Error,
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
    /// Create a new `JSONValue` from an input string
    ///
    /// This is the primary method of constructing a [`JSONValue`]. It cannot fail, although the
    /// value might have type [`JSONValueType::Error`]. However, a malformed payload may have a
    /// type that is not `JSONValueType::Error`.
    ///
    /// If you want to load the payload and verify that it is valid JSON, use
    /// [`JSONValue::load_and_verify`].
    pub fn load(contents: &'a str) -> JSONValue {
        let (contents, _) = trim_start(contents);
        let value_type = JSONValue::peek_value_type(contents);
        JSONValue {
            contents,
            value_type,
        }
    }

    /// Guess the type of the JSON variable serialised in the input string
    ///
    /// This function will never give the _wrong_ type, though it may return a type even if the
    /// input string is not well formed.
    fn peek_value_type(contents: &'a str) -> JSONValueType {
        // The contents must be trimmed
        match contents.chars().next() {
            Some('{') => JSONValueType::Object,
            Some('[') => JSONValueType::Array,
            Some('"') => JSONValueType::String,
            Some('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '-') => {
                JSONValueType::Number
            }
            Some('t' | 'f') => JSONValueType::Bool,
            Some('n') => JSONValueType::Null,
            _ => JSONValueType::Error,
        }
    }

    /// Confirm that this [`JSONValue`] is proper JSON
    ///
    /// This will scan through the entire JSON and confirm that it is properly formatted.
    /// See also [`JSONValue::load_and_verify`].
    ///
    /// ## Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::load("[1,{},\"foo\"]");
    /// assert!(value.verify().is_ok());
    ///
    /// let value = JSONValue::load("[,,{\"");
    /// assert!(value.verify().is_err());
    /// ```
    pub fn verify(&self) -> Result<(), JSONParsingError> {
        JSONValue::parse_with_len(self.contents)?;
        Ok(())
    }

    /// Load a JSON value from a payload and verify that it is valid JSON.
    ///
    /// This is equivalent to calling [`JSONValue::load`] followed by [`JSONValue::verify`].
    pub fn load_and_verify(contents: &'a str) -> Result<JSONValue, JSONParsingError> {
        let value = JSONValue::load(contents);
        value.verify()?;
        Ok(value)
    }

    fn parse_with_len(contents: &'a str) -> Result<(JSONValue, usize), JSONParsingError> {
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
                        return Err(JSONParsingError::CannotParseString);
                    }
                    let (new_contents, whitespace) = trim_start(&contents[item_len..]);
                    contents = new_contents;
                    value_len += item_len + whitespace;
                    if contents.is_empty() {
                        return Err(JSONParsingError::EndOfStream);
                    } else if contents.starts_with(':') {
                        value_len += 1;
                        contents = &contents[1..];
                    } else {
                        return Err(JSONParsingError::UnexpectedToken);
                    }

                    let (_, item_len) = JSONValue::parse_with_len(contents)?;
                    let (new_contents, whitespace) = trim_start(&contents[item_len..]);
                    contents = new_contents;
                    value_len += item_len + whitespace;
                    if contents.is_empty() {
                        return Err(JSONParsingError::EndOfStream);
                    } else if contents.starts_with(',') {
                        value_len += 1;
                        contents = &contents[1..];
                    } else if !contents.starts_with('}') {
                        return Err(JSONParsingError::UnexpectedToken);
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
                        return Err(JSONParsingError::EndOfStream);
                    } else if contents.starts_with(',') {
                        value_len += 1;
                        contents = &contents[1..];
                    } else if !contents.starts_with(']') {
                        return Err(JSONParsingError::UnexpectedToken);
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
                    return Err(JSONParsingError::UnexpectedToken);
                }
                (JSONValueType::Bool, 4)
            }
            Some('f') => {
                if &contents[..5] != "false" {
                    return Err(JSONParsingError::UnexpectedToken);
                }
                (JSONValueType::Bool, 5)
            }
            Some('n') => {
                if &contents[..4] != "null" {
                    return Err(JSONParsingError::UnexpectedToken);
                }
                (JSONValueType::Null, 4)
            }
            _ => {
                return Err(JSONParsingError::UnexpectedToken);
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
    /// # use microjson::{JSONValue, JSONParsingError};
    /// let value = JSONValue::load("-24");
    /// assert_eq!(value.read_integer(), Ok(-24));
    ///
    /// let value = JSONValue::load("5pi");
    /// assert_eq!(value.read_integer(), Err(JSONParsingError::CannotParseInteger));
    /// ```
    pub fn read_integer(&self) -> Result<isize, JSONParsingError> {
        if self.value_type != JSONValueType::Number {
            return Err(JSONParsingError::CannotParseInteger);
        }
        let contents = self.contents.trim_end();
        str::parse(contents).map_err(|_| JSONParsingError::CannotParseInteger)
    }

    /// Reads the [`JSONValue`] as a float
    ///
    /// If the type is not a [`JSONValueType::Number`], returns an `Err`.
    ///
    /// ### Example
    /// ```
    /// # use microjson::{JSONValue, JSONParsingError};
    /// let value = JSONValue::load("2.4");
    /// assert_eq!(value.read_float(), Ok(2.4));
    ///
    /// let value = JSONValue::load("5pi");
    /// assert_eq!(value.read_float(), Err(JSONParsingError::CannotParseFloat));
    /// ```
    pub fn read_float(&self) -> Result<f32, JSONParsingError> {
        if self.value_type != JSONValueType::Number {
            return Err(JSONParsingError::CannotParseFloat);
        }
        let contents = self.contents.trim_end();
        str::parse(contents).map_err(|_| JSONParsingError::CannotParseFloat)
    }

    /// Read the [`JSONValue`] as a string
    ///
    /// This returns an unescaped string (actually a slice into the underlying bytes). If you need
    /// escape sequences to be handled, use [`JSONValue::iter_string`].
    ///
    /// ## Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::load("\"this is a string\"");
    /// assert_eq!(value.read_string(), Ok("this is a string"));
    /// ```
    pub fn read_string(&self) -> Result<&'a str, JSONParsingError> {
        let (_, length) = JSONValue::parse_with_len(self.contents)?;
        if self.value_type != JSONValueType::String {
            return Err(JSONParsingError::CannotParseString);
        }
        Ok(&self.contents[1..length - 1])
    }

    /// Constructs an iterator over this array value
    ///
    /// If the value is not an [`JSONValueType::Array`], returns an error.
    pub fn iter_array(&self) -> Result<JSONArrayIterator<'a>, JSONParsingError> {
        if self.value_type != JSONValueType::Array {
            return Err(JSONParsingError::CannotParseArray);
        }
        Ok(JSONArrayIterator {
            contents: &self.contents[1..],
        })
    }

    /// Constructs an iterator over this string
    ///
    /// If the value is not an [`JSONValueType::String`], returns an error.
    ///
    /// The iterator returns [`Result<char, JSONParsingError>`]s and handles escape sequences.
    /// You can convert this into a `Result<String, _>` using `collect`.
    ///
    /// ### Example
    /// ```
    /// # use microjson::JSONValue;
    /// let value = JSONValue::load(r#" "\u27FC This is a string with unicode \u27FB""#);
    /// let string : Result<String, _> = value.iter_string().unwrap().collect::<Result<String, _>>();
    /// assert_eq!(string.unwrap(), "⟼ This is a string with unicode ⟻")
    /// ```
    pub fn iter_string(&self) -> Result<EscapedStringIterator<'a>, JSONParsingError> {
        if self.value_type != JSONValueType::String {
            return Err(JSONParsingError::CannotParseString);
        }
        Ok(EscapedStringIterator {
            contents: self.contents[1..].chars(),
            done: false,
        })
    }

    /// Constructs an iterator over this object
    ///
    /// If the value is not an [`JSONValueType::Object`], returns an error.
    pub fn iter_object(&self) -> Result<JSONObjectIterator<'a>, JSONParsingError> {
        if self.value_type != JSONValueType::Object {
            return Err(JSONParsingError::CannotParseObject);
        }
        Ok(JSONObjectIterator {
            contents: &self.contents[1..],
        })
    }

    /// Searches this object for a key and returns it's value
    ///
    /// Like the function [`Iterator::nth`], this searches linearly through all the keys in the
    /// object to find the desired one. If parsing the entire object in an arbitrary order, then,
    /// prefer using [`JSONValue::iter_object`].
    ///
    /// Will return `Err(JSONParsingError::KeyNotFound)` if the key is not present.
    pub fn get_key_value(&self, key: &str) -> Result<JSONValue, JSONParsingError> {
        self.iter_object()?
            .find(|item| matches!(item, Ok((k, _)) if k == &key))
            .map(|item| item.unwrap().1)
            .ok_or(JSONParsingError::KeyNotFound)
    }
}

/// An iterator through a JSON object
///
/// Usually constructed with [`JSONValue::iter_object`].
///
/// The iterator items are `Result<(key, value), JSONParsingError>`, but the key is not escaped
pub struct JSONObjectIterator<'a> {
    contents: &'a str,
}

impl<'a> Iterator for JSONObjectIterator<'a> {
    type Item = Result<(&'a str, JSONValue<'a>), JSONParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.contents = self.contents.trim_start();
        if self.contents.is_empty() {
            None
        } else {
            if !self.contents.starts_with('\"') {
                self.contents = &self.contents[..0];
                return None;
            }
            // We expect this to be a string value for the key
            match JSONValue::parse_with_len(self.contents) {
                Ok((_, key_len)) => {
                    let this_key = &self.contents[1..key_len - 1];
                    self.contents = &self.contents[key_len..].trim_start()[1..];

                    match JSONValue::parse_with_len(self.contents) {
                        Ok((this_value, value_len)) => {
                            self.contents = &self.contents[value_len..].trim_start();
                            if !self.contents.is_empty() {
                                self.contents = &self.contents[1..];
                            }
                            Some(Ok((this_key, this_value)))
                        }
                        Err(e) => {
                            self.contents = &self.contents[..0];
                            Some(Err(e))
                        }
                    }
                }
                Err(e) => {
                    self.contents = &self.contents[..0];
                    Some(Err(e))
                }
            }
        }
    }
}

/// An iterator through a JSON array value
///
/// Usually constructed with [`JSONValue::iter_array`].
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

/// Iterator over a JSON-escaped string
///
/// See [`JSONValue::iter_string`] for further documentation.
pub struct EscapedStringIterator<'a> {
    contents: core::str::Chars<'a>,
    done: bool,
}

impl<'a> Iterator for EscapedStringIterator<'a> {
    type Item = Result<char, JSONParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let chr = self.contents.next();
            match chr {
                Some('\\') => {
                    let chr = self.contents.next();
                    match chr {
                        Some('"' | '\\' | '/') => chr.map(Ok),
                        Some('b') => Some(Ok('\x08')),
                        Some('f') => Some(Ok('\x0c')),
                        Some('n') => Some(Ok('\n')),
                        Some('t') => Some(Ok('\t')),
                        Some('r') => Some(Ok('\r')),
                        Some('u') => {
                            let mut get_digit = || {
                                self.contents
                                    .next()
                                    .and_then(|x| x.to_digit(16))
                                    .ok_or(JSONParsingError::TooShortEscapeSequence)
                            };
                            let mut parse_unicode = || {
                                let code = [get_digit()?, get_digit()?, get_digit()?, get_digit()?];
                                let code =
                                    (code[0] << 12) | (code[1] << 8) | (code[2] << 4) | code[3];
                                char::from_u32(code)
                                    .ok_or(JSONParsingError::InvalidUnicodeEscapeSequence)
                            };
                            match parse_unicode() {
                                Ok(chr) => Some(Ok(chr)),
                                Err(e) => {
                                    self.done = true;
                                    Some(Err(e))
                                }
                            }
                        }
                        Some(x) => {
                            self.done = true;
                            Some(Err(JSONParsingError::InvalidEscapeSequence(x)))
                        }
                        None => None,
                    }
                }
                Some('"') => {
                    self.done = true;
                    None
                }
                None => {
                    self.done = true;
                    Some(Err(JSONParsingError::EndOfStream))
                }
                _ => chr.map(Ok),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    extern crate std;

    #[test]
    fn integer() {
        let (value, value_len) = JSONValue::parse_with_len("42").unwrap();
        assert_eq!(value.value_type, JSONValueType::Number);
        assert_eq!(value_len, 2);
        assert_eq!(value.read_integer(), Ok(42));
        assert!(value.read_string().is_err());

        assert_eq!(JSONValue::load("-98").read_integer(), Ok(-98));
        assert_eq!(JSONValue::load("-99 ").read_integer(), Ok(-99));
    }

    #[test]
    fn float() {
        let (value, value_len) = JSONValue::parse_with_len("3.141592").unwrap();
        assert_eq!(value.value_type, JSONValueType::Number);
        assert_eq!(value_len, "3.141592".len());
        assert_eq!(
            value.read_integer(),
            Err(JSONParsingError::CannotParseInteger)
        );
        assert_eq!(
            value.read_string(),
            Err(JSONParsingError::CannotParseString)
        );
        assert!((value.read_float().unwrap() - 3.141592).abs() < 0.0001);

        assert_eq!(
            JSONValue::load("-3.43w").read_float(),
            Err(JSONParsingError::CannotParseFloat)
        );
    }

    #[test]
    fn string() {
        let (value, value_len) = JSONValue::parse_with_len("\"hello world\"").unwrap();
        assert_eq!(value.value_type, JSONValueType::String);
        assert_eq!(value_len, "\"hello world\"".len());
        assert!(value.read_integer().is_err());
        assert_eq!(value.read_string(), Ok("hello world"));

        let value = JSONValue::load("\"hello world\"   ");
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
        \"name\": \"Ginger Fuller\"}";
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
        assert_eq!(
            value.get_key_value("surname").err(),
            Some(JSONParsingError::KeyNotFound)
        );

        assert!(JSONValue::load("{\"foo\":[{}]}").value_type != JSONValueType::Error);
        assert!(JSONValue::load("[{\"foo\":{}}]").value_type != JSONValueType::Error);
    }
    #[test]

    fn this_broke_once() {
        assert!(
            JSONValue::load(
                r##"
[{"a":{"email":"d@"},"m":"#20\n\n.\n"}]
    "##
            )
            .value_type
                != JSONValueType::Error
        )
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
        assert_eq!(JSONValue::peek_value_type("123"), JSONValueType::Number);
        assert_eq!(JSONValue::peek_value_type("12.3"), JSONValueType::Number);
        assert_eq!(JSONValue::peek_value_type("12.3e10"), JSONValueType::Number);
        assert_eq!(JSONValue::peek_value_type("-3"), JSONValueType::Number);
        assert_eq!(JSONValue::peek_value_type("-3.5"), JSONValueType::Number);
        assert_eq!(JSONValue::peek_value_type("null"), JSONValueType::Null);
        assert_eq!(JSONValue::peek_value_type("true"), JSONValueType::Bool);
        assert_eq!(JSONValue::peek_value_type("false"), JSONValueType::Bool);
        assert_eq!(JSONValue::peek_value_type("[]"), JSONValueType::Array);
        assert_eq!(JSONValue::peek_value_type("[12]"), JSONValueType::Array);
        assert_eq!(JSONValue::peek_value_type("[1,2]"), JSONValueType::Array);
        assert_eq!(JSONValue::peek_value_type("[[]]"), JSONValueType::Array);
        assert_eq!(JSONValue::peek_value_type("\"foo\""), JSONValueType::String);
        assert_eq!(JSONValue::peek_value_type("{}"), JSONValueType::Object);
        assert_eq!(
            JSONValue::peek_value_type("{\"a\":2}"),
            JSONValueType::Object
        );
        assert_eq!(JSONValue::peek_value_type("<"), JSONValueType::Error);
        assert_eq!(JSONValue::peek_value_type("bar"), JSONValueType::Error);
    }

    #[test]
    fn verifying() {
        assert!(JSONValue::load_and_verify(" 123 ").is_ok());
        assert!(JSONValue::load_and_verify("[123]").is_ok());
        assert!(JSONValue::load_and_verify("\"foo\"").is_ok());
    }

    #[test]
    fn string_iterator() {
        let try_parse_string = |s| {
            JSONValue::load(s)
                .iter_string()
                .unwrap()
                .collect::<Result<std::string::String, _>>()
        };
        let value = try_parse_string("\"I have a dream\"").unwrap();
        assert_eq!(value, "I have a dream");

        let value = try_parse_string("\"\\\"I have a dream\\\"\"").unwrap();
        assert_eq!(value, "\"I have a dream\"");

        let value = try_parse_string(r#" "\"I\n\thave\b\fa\\dream\/\"\u00a3" "#).unwrap();
        assert_eq!(value, "\"I\n\thave\x08\x0ca\\dream/\"£");

        let value = try_parse_string(r#" " "#);
        assert!(matches!(value, Err(JSONParsingError::EndOfStream)));
        let value = try_parse_string(r#" "foo\" "#);
        assert!(matches!(value, Err(JSONParsingError::EndOfStream)));
        let value = try_parse_string(r#" "Odd escape: \?" "#);
        assert!(matches!(
            value,
            Err(JSONParsingError::InvalidEscapeSequence('?'))
        ));
        let value = try_parse_string(r#" "\uwxyz" "#);
        assert!(matches!(
            value,
            Err(JSONParsingError::TooShortEscapeSequence)
        ));
        let value = try_parse_string(r#" "\u012" "#);
        assert!(matches!(
            value,
            Err(JSONParsingError::TooShortEscapeSequence)
        ));
        // This is not a single character codepoint under utf-16
        let value = try_parse_string(r#" "\ud834" "#);
        assert!(matches!(
            value,
            Err(JSONParsingError::InvalidUnicodeEscapeSequence)
        ));
    }

    #[test]
    fn object_iterator() {
        let json_value = JSONValue::load("{\"foo\" : [], \"bar\":{\"baz\": 2}}");
        let keys = ["foo", "bar"];
        for (item, expected_key) in json_value.iter_object().unwrap().zip(&keys) {
            assert_eq!(item.unwrap().0, *expected_key);
        }
    }

    #[test]
    fn string_borrow_past_lifetime_of_value() {
        let s = "\"abc\"";
        let t: &str;
        {
            t = JSONValue::load(s).read_string().unwrap();
        }
        assert_eq!(t, &s[1..s.len()-1]);
    }
}
