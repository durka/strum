//! # Strum
//!
//! [![Build Status](https://travis-ci.org/Peternator7/strum.svg?branch=master)](https://travis-ci.org/Peternator7/strum)
//! [![Latest Version](https://img.shields.io/crates/v/strum.svg)](https://crates.io/crates/strum)
//! [![Rust Documentation](https://docs.rs/strum/badge.svg)](https://docs.rs/strum)
//!
//! Strum is a set of macros and traits for working with
//! enums and strings easier in Rust.
//!
//! # Including Strum in Your Project
//!
//! Import strum and strum_macros into your project by adding the following lines to your
//! Cargo.toml. Strum_macros contains the macros needed to derive all the traits in Strum.
//!
//! ```toml
//! [dependencies]
//! strum = "0.8.0"
//! strum_macros = "0.8.0"
//! ```
//!
//! And add these lines to the root of your project, either lib.rs or main.rs.
//!
//! ```rust
//! // Strum contains all the trait definitions
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//! # fn main() {}
//! ```
//!
//! # Strum Macros
//!
//! Strum has implemented the following macros:
//!
//! 1. `EnumString`: auto-derives `std::str::FromStr` on the enum. Each variant of the enum will match on it's
//!     own name. This can be overridden using `serialize="DifferentName"` or `to_string="DifferentName"`on the attribute as shown below.
//!     Multiple deserializations can be added to the same variant. If the variant contains additional data,
//!     they will be set to their default values upon deserialization.
//!
//!     The `default` attribute can be applied to a tuple variant with a single data parameter. When a match isn't
//!     found, the given variant will be returned and the input string will be captured in the parameter.
//!
//!     Here is an example of the code generated by deriving `EnumString`.
//!
//!     ```
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
//!     #[derive(EnumString)]
//!     enum Color {
//!         Red,
//!
//!         // The Default value will be inserted into range if we match "Green".
//!         Green { range:usize },
//!
//!         // We can match on multiple different patterns.
//!         #[strum(serialize="blue",serialize="b")]
//!         Blue(usize),
//!
//!         // Notice that we can disable certain variants from being found
//!         #[strum(disabled="true")]
//!         Yellow,
//!     }
//!
//!     /*
//!     //The generated code will look like:
//!     impl ::std::str::FromStr for Color {
//!         type Err = ::strum::ParseError;
//!
//!         fn from_str(s: &str) -> ::std::result::Result<Color, Self::Err> {
//!             match s {
//!                 "Red" => ::std::result::Result::Ok(Color::Red),
//!                 "Green" => ::std::result::Result::Ok(Color::Green { range:Default::default() }),
//!                 "blue" | "b" => ::std::result::Result::Ok(Color::Blue(Default::default())),
//!                 _ => ::std::result::Result::Err(strum::ParseError::VariantNotFound),
//!             }
//!         }
//!     }
//!     */
//!     # fn main() {}
//!     ```
//!
//!     Note that the implementation of `FromStr` only matches on the name of the variant.
//!     Strum, where possible, avoids operations that have an unknown runtime cost, and parsing strings
//!     is potentially an expensive operation. If you do need that behavior, consider the more powerful
//!     Serde library for your serialization.
//!
//! 2. `ToString`: prints out the given enum variant as a string. This enables you to perform round trip
//!    style conversions from enum into string and back again for unit style variants. `ToString` chooses
//!    which serialization to used based on the following criteria:
//!
//!    1. If there is a `to_string` property, this value will be used. There can only be one per variant.
//!    2. Of the various `serialize` properties, the value with the longest length is chosen. If that
//!       behavior isn't desired, you should use `to_string`.
//!    3. The name of the variant will be used if there are no `serialize` or `to_string` attributes.
//!
//!    ```rust
//!    # extern crate strum;
//!    # #[macro_use] extern crate strum_macros;
//!    // You need to bring the type into scope to use it!!!
//!    use std::string::ToString;
//!
//!    #[derive(ToString, Debug)]
//!    enum Color {
//!        #[strum(serialize="redred")]
//!        Red,
//!        Green { range:usize },
//!        Blue(usize),
//!        Yellow,
//!    }
//!
//!    // It's simple to iterate over the variants of an enum.
//!    fn debug_colors() {
//!        let red = Color::Red;
//!        assert_eq!(String::from("redred"), red.to_string());
//!    }
//!
//!    fn main () { debug_colors(); }
//!    ```
//!
//! 3. `AsRefStr`: this derive implements `AsRef<str>` on your enum using the same rules as
//!    `ToString` for determining what string is returned. The difference is that `as_ref()` returns
//!     a `&str` instead of a `String` so you don't allocate any additional memory with each call.
//!
//! 4. `EnumIter`: iterate over the variants of an Enum. Any additional data on your variants will be
//!     set to `Default::default()`. The macro implements `strum::IntoEnumIter` on your enum and
//!     creates a new type called `YourEnumIter` that is the iterator object. You cannot derive
//!     `EnumIter` on any type with a lifetime bound (`<'a>`) because the iterator would surely
//!     create [unbounded lifetimes] (https://doc.rust-lang.org/nightly/nomicon/unbounded-lifetimes.html).
//!
//!     ```rust
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
//!     # use std::fmt::Debug;
//!     // You need to bring the type into scope to use it!!!
//!     use strum::IntoEnumIterator;
//!
//!     #[derive(EnumIter,Debug)]
//!     enum Color {
//!         Red,
//!         Green { range:usize },
//!         Blue(usize),
//!         Yellow,
//!     }
//!
//!     // It's simple to iterate over the variants of an enum.
//!     fn debug_colors() {
//!         for color in Color::iter() {
//!             println!("My favorite color is {:?}", color);
//!         }
//!     }
//!
//!     fn main() {
//!         debug_colors();
//!     }
//!     ```
//!
//! 5. `EnumMessage`: encode strings into the enum itself. This macro implements
//!     the `strum::EnumMessage` trait. `EnumMessage` looks for
//!     `#[strum(message="...")]` attributes on your variants.
//!     You can also provided a `detailed_message="..."` attribute to create a
//!     seperate more detailed message than the first.
//!
//!     The generated code will look something like:
//!
//!     ```rust
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
//!     // You need to bring the type into scope to use it!!!
//!     use strum::EnumMessage;
//!
//!     #[derive(EnumMessage,Debug)]
//!     enum Color {
//!         #[strum(message="Red",detailed_message="This is very red")]
//!         Red,
//!         #[strum(message="Simply Green")]
//!         Green { range:usize },
//!         #[strum(serialize="b",serialize="blue")]
//!         Blue(usize),
//!     }
//!
//!     /*
//!     // Generated code
//!     impl ::strum::EnumMessage for Color {
//!         fn get_message(&self) -> ::std::option::Option<&str> {
//!             match self {
//!                 &Color::Red => ::std::option::Option::Some("Red"),
//!                 &Color::Green {..} => ::std::option::Option::Some("Simply Green"),
//!                 _ => None
//!             }
//!         }
//!
//!         fn get_detailed_message(&self) -> ::std::option::Option<&str> {
//!             match self {
//!                 &Color::Red => ::std::option::Option::Some("This is very red"),
//!                 &Color::Green {..}=> ::std::option::Option::Some("Simply Green"),
//!                 _ => None
//!             }
//!         }
//!
//!         fn get_serializations(&self) -> &[&str] {
//!             match self {
//!                 &Color::Red => {
//!                     static ARR: [&'static str; 1] = ["Red"];
//!                     &ARR
//!                 },
//!                 &Color::Green {..}=> {
//!                     static ARR: [&'static str; 1] = ["Green"];
//!                     &ARR
//!                 },
//!                 &Color::Blue (..) => {
//!                     static ARR: [&'static str; 2] = ["b", "blue"];
//!                     &ARR
//!                 },
//!             }
//!         }
//!     }
//!     */
//!     # fn main() {}
//!     ```
//!
//! 6. `EnumProperty`: Enables the encoding of arbitary constants into enum variants. This method
//!     currently only supports adding additional string values. Other types of literals are still
//!     experimental in the rustc compiler. The generated code works by nesting match statements.
//!     The first match statement matches on the type of the enum, and the inner match statement
//!     matches on the name of the property requested. This design works well for enums with a small
//!     number of variants and properties, but scales linearly with the number of variants so may not
//!     be the best choice in all situations.
//!
//!     Here's an example:
//!
//!     ```rust
//!     # extern crate strum;
//!     # #[macro_use] extern crate strum_macros;
//!     # use std::fmt::Debug;
//!     // You need to bring the type into scope to use it!!!
//!     use strum::EnumProperty;
//!
//!     #[derive(EnumProperty,Debug)]
//!     enum Color {
//!         #[strum(props(Red="255",Blue="255",Green="255"))]
//!         White,
//!         #[strum(props(Red="0",Blue="0",Green="0"))]
//!         Black,
//!         #[strum(props(Red="0",Blue="255",Green="0"))]
//!         Blue,
//!         #[strum(props(Red="255",Blue="0",Green="0"))]
//!         Red,
//!         #[strum(props(Red="0",Blue="0",Green="255"))]
//!         Green,
//!     }
//!
//!     fn main() {
//!         let my_color = Color::Red;
//!         let display = format!("My color is {:?}. It's RGB is {},{},{}", my_color
//!                                                , my_color.get_str("Red").unwrap()
//!                                                , my_color.get_str("Green").unwrap()
//!                                                , my_color.get_str("Blue").unwrap());
//!     #    let expected = String::from("My color is Red. It's RGB is 255,0,0");
//!     #    assert_eq!(expected, display);
//!     }
//!     ```
//!
//!
//! # Additional Attributes
//!
//! Strum supports several custom attributes to modify the generated code. Custom attributes are
//! applied to a variant by adding #[strum(parameter="value")] to the variant.
//!
//! - `serialize="..."`: Changes the text that `FromStr()` looks for when parsing a string. This attribute can
//!    be applied multiple times to an element and the enum variant will be parsed if any of them match.
//!
//! - `default="true"`: Applied to a single variant of an enum. The variant must be a Tuple-like
//!    variant with a single piece of data that can be create from a `&str` i.e. `T: From<&str>`.
//!    The generated code will now return the variant with the input string captured as shown below
//!    instead of failing.
//!
//!     ```ignore
//!     // Replaces this:
//!     _ => Err(strum::ParseError::VariantNotFound)
//!     // With this in generated code:
//!     default => Ok(Variant(default.into()))
//!     ```
//!     The plugin will fail if the data doesn't implement From<&str>. You can only have one `default`
//!     on your enum.
//!
//! - `disabled="true"`: removes variant from generated code.
//!
//! - `message=".."`: Adds a message to enum variant. This is used in conjunction with the `EnumMessage`
//!    trait to associate a message with a variant. If `detailed_message` is not provided,
//!    then `message` will also be returned when get_detailed_message() is called.
//!
//! - `detailed_message=".."`: Adds a more detailed message to a variant. If this value is omitted, then
//!    `message` will be used in it's place.
//!
//! - `props(key="value")`: Used by EnumProperty to add additional information to an enum variant. Multiple
//!     properties can be added in a single nested block.
//!
//! # Examples
//!
//! Using `EnumMessage` for quickly implementing `Error`
//!
//! ```rust
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//! # use std::error::Error;
//! # use std::fmt::*;
//! use strum::EnumMessage;
//!
//! #[derive(Debug, EnumMessage)]
//! enum ServerError {
//!     #[strum(message="A network error occured")]
//!     #[strum(detailed_message="Try checking your connection.")]
//!     NetworkError,
//!     #[strum(message="User input error.")]
//!     #[strum(detailed_message="There was an error parsing user input. Please try again.")]
//!     InvalidUserInputError,
//! }
//!
//! impl Display for ServerError {
//!     fn fmt(&self, f: &mut Formatter) -> Result {
//!         write!(f, "{}", self.get_message().unwrap())
//!     }
//! }
//!
//! impl Error for ServerError {
//!     fn description(&self) -> &str {
//!         self.get_detailed_message().unwrap()
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Using `EnumString` to tokenize a series of inputs:
//!
//! ```rust
//! extern crate strum;
//! #[macro_use]
//! extern crate strum_macros;
//! use std::str::FromStr;
//!
//! #[derive(Eq, PartialEq, Debug, EnumString)]
//! enum Tokens {
//!     #[strum(serialize="fn")]
//!     Function,
//!     #[strum(serialize="(")]
//!     OpenParen,
//!     #[strum(serialize=")")]
//!     CloseParen,
//!     #[strum(default="true")]
//!     Ident(String)
//! }
//!
//! fn main() {
//!     let toks = ["fn", "hello_world", "(", ")"].iter()
//!                    .map(|tok| Tokens::from_str(tok).unwrap())
//!                    .collect::<Vec<_>>();
//!
//!     assert_eq!(toks, vec![Tokens::Function,
//!                           Tokens::Ident(String::from("hello_world")),
//!                           Tokens::OpenParen,
//!                           Tokens::CloseParen]);
//! }
//! ```
//!
//! # Debugging
//!
//! To see the generated code, set the STRUM_DEBUG environment variable before compiling your code.
//! `STRUM_DEBUG=1` will dump all of the generated code for every type. `STRUM_DEBUG=YourType` will
//! only dump the code generated on a type named YourType.
//!

/// The ParseError enum is a collection of all the possible reasons
/// an enum can fail to parse from a string.
#[derive(Debug,Clone,Copy,Eq,PartialEq,Hash)]
pub enum ParseError {
    VariantNotFound,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        // We could use our macro here, but this way we don't take a dependency on the
        // macros crate.
        match self {
            &ParseError::VariantNotFound => write!(f, "Matching variant not found"),
        }
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str {
        match self {
            &ParseError::VariantNotFound => {
                "Unable to find a variant of the given enum matching the string given. Matching \
                 can be extended with the Serialize attribute and is case sensitive."
            }
        }
    }
}

/// This trait designates that an `Enum` can be iterated over. It can
/// be auto generated using `strum_macros` on your behalf.
///
/// # Example
///
/// ```rust
/// # extern crate strum;
/// # #[macro_use] extern crate strum_macros;
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::IntoEnumIterator;
///
/// #[derive(EnumIter,Debug)]
/// enum Color {
///         Red,
///         Green { range:usize },
///         Blue(usize),
///         Yellow,
/// }
///
/// // Iterating over any enum requires 2 type parameters
/// // A 3rd is used in this example to allow passing a predicate
/// fn generic_iterator<E, I, F>(pred: F)
///                      where E: IntoEnumIterator<Iterator=I>,
///                            I: Iterator<Item=E>,
///                            F: Fn(E) {
///     for e in E::iter() {
///         pred(e)
///     }
/// }
///
/// fn main() {
///     generic_iterator::<Color,_, _>(|color| println!("{:?}", color));
/// }
/// ```
pub trait IntoEnumIterator {
    type Iterator;

    fn iter() -> Self::Iterator;
}

/// Associates additional pieces of information with an Enum. This can be
/// autoimplemented by deriving `EnumMessage` and annotating your variants with
/// `#[strum(message="...")].
///
/// # Example
///
/// ```rust
/// # extern crate strum;
/// # #[macro_use] extern crate strum_macros;
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::EnumMessage;
///
/// #[derive(PartialEq, Eq, Debug, EnumMessage)]
/// enum Pet {
///     #[strum(message="I have a dog")]
///     #[strum(detailed_message="My dog's name is Spots")]
///     Dog,
///     #[strum(message="I don't have a cat")]
///     Cat,
/// }
///
/// fn main() {
///     let my_pet = Pet::Dog;
///     assert_eq!("I have a dog", my_pet.get_message().unwrap());
/// }
/// ```
pub trait EnumMessage {
    fn get_message(&self) -> Option<&str>;
    fn get_detailed_message(&self) -> Option<&str>;
    fn get_serializations(&self) -> &[&str];
}

/// EnumProperty is a trait that makes it possible to store additional information
/// with enum variants. This trait is designed to be used with the macro of the same
/// name in the `strum_macros` crate. Currently, the only string literals are supported
/// in attributes, the other methods will be implemented as additional attribute types
/// become stabilized.
///
/// # Example
///
/// ```rust
/// # extern crate strum;
/// # #[macro_use] extern crate strum_macros;
/// # use std::fmt::Debug;
/// // You need to bring the type into scope to use it!!!
/// use strum::EnumProperty;
///
/// #[derive(PartialEq, Eq, Debug, EnumProperty)]
/// enum Class {
///     #[strum(props(Teacher="Ms.Frizzle", Room="201"))]
///     History,
///     #[strum(props(Teacher="Mr.Smith"))]
///     #[strum(props(Room="103"))]
///     Mathematics,
///     #[strum(props(Time="2:30"))]
///     Science,
/// }
///
/// fn main() {
///     let history = Class::History;
///     assert_eq!("Ms.Frizzle", history.get_str("Teacher").unwrap());
/// }
/// ```
pub trait EnumProperty {
    fn get_str(&self, &str) -> Option<&'static str>;
    fn get_int(&self, &str) -> Option<usize> {
        Option::None
    }

    fn get_bool(&self, &str) -> Option<bool> {
        Option::None
    }
}
