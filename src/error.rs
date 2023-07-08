/// Errors while parsing JSON
///
/// Due to the "scan once" philosophy of this crate, errors can either be returned when first
/// constructing a [`JSONValue`] or when trying to read it using one of the accessors.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum JSONParsingError {
    /// Attempt to parse an object that is not an array as an array
    CannotParseArray,
    /// Attempt to parse an object that is not a float as a float
    CannotParseFloat,
    /// Attempt to parse an object that is not an integer as an integer
    CannotParseInteger,
    /// Attempt to parse an object that is not an object as an object
    CannotParseObject,
    /// Attempt to parse an object that is not a string as an string
    CannotParseString,
    /// The key is not present in the object
    KeyNotFound,
    /// There was an unexpected token in the input stream
    UnexpectedToken,
    /// The input stream terminated while scanning a type
    EndOfStream,
}

impl core::fmt::Debug for JSONParsingError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::KeyNotFound => {
                write!(f, "key not found")
            }
            Self::EndOfStream => {
                write!(f, "stream ended while parsing JSON")
            }
            Self::UnexpectedToken => {
                write!(f, "unexpected token")
            }
            Self::CannotParseArray => {
                write!(f, "error parsing array")
            }
            Self::CannotParseFloat => {
                write!(f, "error parsing float")
            }
            Self::CannotParseInteger => {
                write!(f, "error parsing integer")
            }
            Self::CannotParseString => {
                write!(f, "error parsing string")
            }
            Self::CannotParseObject => {
                write!(f, "error parsing object")
            }
        }
    }
}
