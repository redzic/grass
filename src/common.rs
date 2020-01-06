use std::convert::TryFrom;
use std::fmt::{self, Display};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Symbol {
    /// .
    Period,
    /// #
    Hash,
    /// @
    At,
    /// $
    Dollar,
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,
    /// ,
    Comma,
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Mul,
    /// /
    Div,
    /// :
    Colon,
    /// ;
    SemiColon,
    /// ~
    Tilde,
    /// >
    Gt,
    /// <
    Lt,
    /// ^
    Xor,
    /// =
    Equal,
    /// |
    BitOr,
    /// &
    BitAnd,
    /// %
    Percent,
    /// "
    DoubleQuote,
    /// '
    SingleQuote,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Period => write!(f, "."),
            Symbol::Hash => write!(f, "#"),
            Symbol::At => write!(f, "@"),
            Symbol::Dollar => write!(f, "$"),
            Symbol::OpenParen => write!(f, "("),
            Symbol::CloseParen => write!(f, "),"),
            Symbol::OpenBrace => write!(f, "{{"),
            Symbol::CloseBrace => write!(f, "}}"),
            Symbol::OpenBracket => write!(f, "["),
            Symbol::CloseBracket => write!(f, "]"),
            Symbol::Comma => write!(f, ","),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Mul => write!(f, "*"),
            Symbol::Div => write!(f, "/"),
            Symbol::Colon => write!(f, ":"),
            Symbol::SemiColon => write!(f, ";"),
            Symbol::Tilde => write!(f, "~"),
            Symbol::Gt => write!(f, ">"),
            Symbol::Lt => write!(f, "<"),
            Symbol::Xor => write!(f, "^"),
            Symbol::Equal => write!(f, "="),
            Symbol::BitOr => write!(f, "|"),
            Symbol::BitAnd => write!(f, "&"),
            Symbol::Percent => write!(f, "%"),
            Symbol::DoubleQuote => write!(f, "\""),
            Symbol::SingleQuote => write!(f, "'"),
        }
    }
}

impl TryFrom<char> for Symbol {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Symbol::Period),
            '#' => Ok(Symbol::Hash),
            '@' => Ok(Symbol::At),
            '$' => Ok(Symbol::Dollar),
            '(' => Ok(Symbol::OpenParen),
            ')' => Ok(Symbol::CloseParen),
            '{' => Ok(Symbol::OpenBrace),
            '}' => Ok(Symbol::CloseBrace),
            '[' => Ok(Symbol::OpenBracket),
            ']' => Ok(Symbol::CloseBracket),
            ',' => Ok(Symbol::Comma),
            '+' => Ok(Symbol::Plus),
            '-' => Ok(Symbol::Minus),
            '*' => Ok(Symbol::Mul),
            '/' => Ok(Symbol::Div),
            ':' => Ok(Symbol::Colon),
            ';' => Ok(Symbol::SemiColon),
            '~' => Ok(Symbol::Tilde),
            '>' => Ok(Symbol::Gt),
            '<' => Ok(Symbol::Lt),
            '^' => Ok(Symbol::Xor),
            '=' => Ok(Symbol::Equal),
            '|' => Ok(Symbol::BitOr),
            '&' => Ok(Symbol::BitAnd),
            '%' => Ok(Symbol::Percent),
            '"' => Ok(Symbol::DoubleQuote),
            '\'' => Ok(Symbol::SingleQuote),
            _ => Err("invalid symbol"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaQuery {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AtRule {
    // SASS specific @rules
    /// Loads mixins, functions, and variables from other Sass stylesheets, and combines CSS from multiple stylesheets together
    Use,
    /// Loads a Sass stylesheet and makes its mixins, functions, and variables available when your stylesheet is loaded with the `@use` rule
    Forward,
    /// Extends the CSS at-rule to load styles, mixins, functions, and variables from other stylesheets
    Import,
    Mixin,
    Include,
    /// Defines custom functions that can be used in SassScript expressions
    Function,
    /// Allows selectors to inherit styles from one another
    Extend,
    /// Puts styles within it at the root of the CSS document
    AtRoot,
    /// Causes compilation to fail with an error message
    Error,
    /// Prints a warning without stopping compilation entirely
    Warn,
    /// Prints a message for debugging purposes
    Debug,
    If,
    Each,
    For,
    While,

    // CSS @rules
    /// Defines the character set used by the style sheet
    Charset,
    /// Tells the CSS engine that all its content must be considered prefixed with an XML namespace
    Namespace,
    /// A conditional group rule that will apply its content if the device meets the criteria of the condition defined using a media query
    Media,
    /// A conditional group rule that will apply its content if the browser meets the criteria of the given condition
    Supports,
    /// Describes the aspect of layout changes that will be applied when printing the document
    Page,
    /// Describes the aspect of an external font to be downloaded
    FontFace,
    /// Describes the aspect of intermediate steps in a CSS animation sequence
    Keyframes,

    // @rules related to @font-feature-values
    FontFeatureValues,
    Swash,
    Ornaments,
    Annotation,
    Stylistic,
    Styleset,
    CharacterVariant,

    // Experimental CSS @rules
    /// Describes the aspects of the viewport for small screen devices
    ///
    /// Currently at the Working Draft stage
    Viewport,
    /// A conditional group rule that will apply its content if the document in which the style sheet is applied meets the criteria of the given condition
    ///
    /// Deferred to Level 4 of CSS Spec
    Document,
    /// Defines specific counter styles that are not part of the predefined set of styles
    ///
    /// At the Candidate Recommendation stage
    CounterStyle,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Whitespace {
    Space,
    Tab,
    Newline,
    CarriageReturn,
}

impl Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Whitespace::Space => write!(f, " "),
            Whitespace::Tab => write!(f, "\t"),
            Whitespace::Newline => writeln!(f),
            Whitespace::CarriageReturn => write!(f, "\r"),
        }
    }
}

impl TryFrom<char> for Whitespace {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            ' ' => Ok(Whitespace::Space),
            '\t' => Ok(Whitespace::Tab),
            '\n' => Ok(Whitespace::Newline),
            '\r' => Ok(Whitespace::CarriageReturn),
            _ => Err("invalid whitespace"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Equal,
    NotEqual,
    GreaterThanEqual,
    LessThanEqual,
}

impl Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Op::Equal => write!(f, "=="),
            Op::NotEqual => write!(f, "!="),
            Op::GreaterThanEqual => write!(f, ">="),
            Op::LessThanEqual => write!(f, "<="),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Important,
    Infinity,
    NaN,
    Auto,
    Inherit,
    Initial,
    Unset,
    True,
    False,
    Not,
    And,
    Or,
    Null,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Important => write!(f, "!important"),
            Keyword::Infinity => write!(f, "Infinity"),
            Keyword::NaN => write!(f, "NaN"),
            Keyword::Auto => write!(f, "auto"),
            Keyword::Inherit => write!(f, "inherit"),
            Keyword::Initial => write!(f, "initial"),
            Keyword::Unset => write!(f, "unset"),
            Keyword::True => write!(f, "true"),
            Keyword::False => write!(f, "false"),
            Keyword::Not => write!(f, "not"),
            Keyword::And => write!(f, "and"),
            Keyword::Or => write!(f, "or"),
            Keyword::Null => write!(f, "null"),
        }
    }
}

impl Into<&'static str> for Keyword {
    fn into(self) -> &'static str {
        match self {
            Keyword::Important => "!important",
            Keyword::Infinity => "Infinity",
            Keyword::NaN => "NaN",
            Keyword::Auto => "auto",
            Keyword::Inherit => "inherit",
            Keyword::Initial => "initial",
            Keyword::Unset => "unset",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Not => "not",
            Keyword::And => "and",
            Keyword::Or => "or",
            Keyword::Null => "null",
        }
    }
}

impl TryFrom<&str> for Keyword {
    type Error = &'static str;

    fn try_from(kw: &str) -> Result<Self, Self::Error> {
        // todo: case insensitive?
        match kw {
            "important" => Ok(Keyword::Important),
            "infinity" => Ok(Keyword::Infinity),
            "nan" => Ok(Keyword::NaN),
            "auto" => Ok(Keyword::Auto),
            "inherit" => Ok(Keyword::Inherit),
            "initial" => Ok(Keyword::Initial),
            "unset" => Ok(Keyword::Unset),
            "true" => Ok(Keyword::True),
            "false" => Ok(Keyword::False),
            "not" => Ok(Keyword::Not),
            "and" => Ok(Keyword::And),
            "or" => Ok(Keyword::Or),
            "null" => Ok(Keyword::Null),
            _ => Err("invalid keyword"),
        }
    }
}

#[derive(Debug)]
pub enum Color {
    AliceBlue,            // = 0xF0F8FF,
    AntiqueWhite,         // = 0xFAEBD7,
    Aqua,                 // = 0x00FFFF,
    Aquamarine,           // = 0x7FFFD4,
    Azure,                // = 0xF0FFFF,
    Beige,                // = 0xF5F5DC,
    Bisque,               // = 0xFFE4C4,
    Black,                // = 0x000000,
    BlanchedAlmond,       // = 0xFFEBCD,
    Blue,                 // = 0x0000FF,
    BlueViolet,           // = 0x8A2BE2,
    Brown,                // = 0xA52A2A,
    BurlyWood,            // = 0xDEB887,
    CadetBlue,            // = 0x5F9EA0,
    Chartreuse,           // = 0x7FFF00,
    Chocolate,            // = 0xD2691E,
    Coral,                // = 0xFF7F50,
    CornflowerBlue,       // = 0x6495ED,
    Cornsilk,             // = 0xFFF8DC,
    Crimson,              // = 0xDC143C,
    Cyan,                 //0x00FFFF
    DarkBlue,             // = 0x00008B,
    DarkCyan,             // = 0x008B8B,
    DarkGoldenRod,        // = 0xB8860B,
    DarkGray,             // = 0xA9A9A9,
    DarkGrey,             //0xA9A9A9
    DarkGreen,            // = 0x006400,
    DarkKhaki,            // = 0xBDB76B,
    DarkMagenta,          // = 0x8B008B,
    DarkOliveGreen,       // = 0x556B2F,
    DarkOrange,           // = 0xFF8C00,
    DarkOrchid,           // = 0x9932CC,
    DarkRed,              // = 0x8B0000,
    DarkSalmon,           // = 0xE9967A,
    DarkSeaGreen,         // = 0x8FBC8F,
    DarkSlateBlue,        // = 0x483D8B,
    DarkSlateGray,        // = 0x2F4F4F,
    DarkSlateGrey,        //0x2F4F4F
    DarkTurquoise,        // = 0x00CED1,
    DarkViolet,           // = 0x9400D3,
    DeepPink,             // = 0xFF1493,
    DeepSkyBlue,          // = 0x00BFFF,
    DimGray,              // = 0x696969,
    DimGrey,              //0x696969
    DodgerBlue,           // = 0x1E90FF,
    FireBrick,            // = 0xB22222,
    FloralWhite,          // = 0xFFFAF0,
    ForestGreen,          // = 0x228B22,
    Fuchsia,              // = 0xFF00FF,
    Gainsboro,            // = 0xDCDCDC,
    GhostWhite,           // = 0xF8F8FF,
    Gold,                 // = 0xFFD700,
    GoldenRod,            // = 0xDAA520,
    Gray,                 // = 0x808080,
    Grey,                 //0x808080
    Green,                // = 0x008000,
    GreenYellow,          // = 0xADFF2F,
    HoneyDew,             // = 0xF0FFF0,
    HotPink,              // = 0xFF69B4,
    IndianRed,            // = 0xCD5C5C,
    Indigo,               // = 0x4B0082,
    Ivory,                // = 0xFFFFF0,
    Khaki,                // = 0xF0E68C,
    Lavender,             // = 0xE6E6FA,
    LavenderBlush,        // = 0xFFF0F5,
    LawnGreen,            // = 0x7CFC00,
    LemonChiffon,         // = 0xFFFACD,
    LightBlue,            // = 0xADD8E6,
    LightCoral,           // = 0xF08080,
    LightCyan,            // = 0xE0FFFF,
    LightGoldenRodYellow, // = 0xFAFAD2,
    LightGray,            // = 0xD3D3D3,
    LightGrey,            //0xD3D3D3
    LightGreen,           // = 0x90EE90,
    LightPink,            // = 0xFFB6C1,
    LightSalmon,          // = 0xFFA07A,
    LightSeaGreen,        // = 0x20B2AA,
    LightSkyBlue,         // = 0x87CEFA,
    LightSlateGray,       // = 0x778899,
    LightSlateGrey,       //0x778899
    LightSteelBlue,       // = 0xB0C4DE,
    LightYellow,          // = 0xFFFFE0,
    Lime,                 // = 0x00FF00,
    LimeGreen,            // = 0x32CD32,
    Linen,                // = 0xFAF0E6,
    Magenta,              //0xFF00FF
    Maroon,               // = 0x800000,
    MediumAquaMarine,     // = 0x66CDAA,
    MediumBlue,           // = 0x0000CD,
    MediumOrchid,         // = 0xBA55D3,
    MediumPurple,         // = 0x9370DB,
    MediumSeaGreen,       // = 0x3CB371,
    MediumSlateBlue,      // = 0x7B68EE,
    MediumSpringGreen,    // = 0x00FA9A,
    MediumTurquoise,      // = 0x48D1CC,
    MediumVioletRed,      // = 0xC71585,
    MidnightBlue,         // = 0x191970,
    MintCream,            // = 0xF5FFFA,
    MistyRose,            // = 0xFFE4E1,
    Moccasin,             // = 0xFFE4B5,
    NavajoWhite,          // = 0xFFDEAD,
    Navy,                 // = 0x000080,
    OldLace,              // = 0xFDF5E6,
    Olive,                // = 0x808000,
    OliveDrab,            // = 0x6B8E23,
    Orange,               // = 0xFFA500,
    OrangeRed,            // = 0xFF4500,
    Orchid,               // = 0xDA70D6,
    PaleGoldenRod,        // = 0xEEE8AA,
    PaleGreen,            // = 0x98FB98,
    PaleTurquoise,        // = 0xAFEEEE,
    PaleVioletRed,        // = 0xDB7093,
    PapayaWhip,           // = 0xFFEFD5,
    PeachPuff,            // = 0xFFDAB9,
    Peru,                 // = 0xCD853F,
    Pink,                 // = 0xFFC0CB,
    Plum,                 // = 0xDDA0DD,
    PowderBlue,           // = 0xB0E0E6,
    Purple,               // = 0x800080,
    RebeccaPurple,        // = 0x663399,
    Red,                  // = 0xFF0000,
    RosyBrown,            // = 0xBC8F8F,
    RoyalBlue,            // = 0x4169E1,
    SaddleBrown,          // = 0x8B4513,
    Salmon,               // = 0xFA8072,
    SandyBrown,           // = 0xF4A460,
    SeaGreen,             // = 0x2E8B57,
    SeaShell,             // = 0xFFF5EE,
    Sienna,               // = 0xA0522D,
    Silver,               // = 0xC0C0C0,
    SkyBlue,              // = 0x87CEEB,
    SlateBlue,            // = 0x6A5ACD,
    SlateGray,            // = 0x708090,
    SlateGrey,            //0x708090
    Snow,                 // = 0xFFFAFA,
    SpringGreen,          // = 0x00FF7F,
    SteelBlue,            // = 0x4682B4,
    Tan,                  // = 0xD2B48C,
    Teal,                 // = 0x008080,
    Thistle,              // = 0xD8BFD8,
    Tomato,               // = 0xFF6347,
    Turquoise,            // = 0x40E0D0,
    Violet,               // = 0xEE82EE,
    Wheat,                // = 0xF5DEB3,
    White,                // = 0xFFFFFF,
    WhiteSmoke,           // = 0xF5F5F5,
    Yellow,               // = 0xFFFF00,
    YellowGreen,          // = 0x9ACD32,
    Other(u32),
}

impl TryFrom<&str> for Color {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        match string {
            "aliceblue" => Ok(Color::AliceBlue),
            "antiquewhite" => Ok(Color::AntiqueWhite),
            "aqua" => Ok(Color::Aqua),
            "aquamarine" => Ok(Color::Aquamarine),
            "azure" => Ok(Color::Azure),
            "beige" => Ok(Color::Beige),
            "bisque" => Ok(Color::Bisque),
            "black" => Ok(Color::Black),
            "blanchedalmond" => Ok(Color::BlanchedAlmond),
            "blue" => Ok(Color::Blue),
            "blueviolet" => Ok(Color::BlueViolet),
            "brown" => Ok(Color::Brown),
            "burlywood" => Ok(Color::BurlyWood),
            "cadetblue" => Ok(Color::CadetBlue),
            "chartreuse" => Ok(Color::Chartreuse),
            "chocolate" => Ok(Color::Chocolate),
            "coral" => Ok(Color::Coral),
            "cornflowerblue" => Ok(Color::CornflowerBlue),
            "cornsilk" => Ok(Color::Cornsilk),
            "crimson" => Ok(Color::Crimson),
            "cyan" => Ok(Color::Cyan),
            "darkblue" => Ok(Color::DarkBlue),
            "darkcyan" => Ok(Color::DarkCyan),
            "darkgoldenrod" => Ok(Color::DarkGoldenRod),
            "darkgray" => Ok(Color::DarkGray),
            "darkgrey" => Ok(Color::DarkGrey),
            "darkgreen" => Ok(Color::DarkGreen),
            "darkkhaki" => Ok(Color::DarkKhaki),
            "darkmagenta" => Ok(Color::DarkMagenta),
            "darkolivegreen" => Ok(Color::DarkOliveGreen),
            "darkorange" => Ok(Color::DarkOrange),
            "darkorchid" => Ok(Color::DarkOrchid),
            "darkred" => Ok(Color::DarkRed),
            "darksalmon" => Ok(Color::DarkSalmon),
            "darkseagreen" => Ok(Color::DarkSeaGreen),
            "darkslateblue" => Ok(Color::DarkSlateBlue),
            "darkslategray" => Ok(Color::DarkSlateGray),
            "darkslategrey" => Ok(Color::DarkSlateGrey),
            "darkturquoise" => Ok(Color::DarkTurquoise),
            "darkviolet" => Ok(Color::DarkViolet),
            "deeppink" => Ok(Color::DeepPink),
            "deepskyblue" => Ok(Color::DeepSkyBlue),
            "dimgray" => Ok(Color::DimGray),
            "dimgrey" => Ok(Color::DimGrey),
            "dodgerblue" => Ok(Color::DodgerBlue),
            "firebrick" => Ok(Color::FireBrick),
            "floralwhite" => Ok(Color::FloralWhite),
            "forestgreen" => Ok(Color::ForestGreen),
            "fuchsia" => Ok(Color::Fuchsia),
            "gainsboro" => Ok(Color::Gainsboro),
            "ghostwhite" => Ok(Color::GhostWhite),
            "gold" => Ok(Color::Gold),
            "goldenrod" => Ok(Color::GoldenRod),
            "gray" => Ok(Color::Gray),
            "grey" => Ok(Color::Grey),
            "green" => Ok(Color::Green),
            "greenyellow" => Ok(Color::GreenYellow),
            "honeydew" => Ok(Color::HoneyDew),
            "hotpink" => Ok(Color::HotPink),
            "indianred" => Ok(Color::IndianRed),
            "indigo" => Ok(Color::Indigo),
            "ivory" => Ok(Color::Ivory),
            "khaki" => Ok(Color::Khaki),
            "lavender" => Ok(Color::Lavender),
            "lavenderblush" => Ok(Color::LavenderBlush),
            "lawngreen" => Ok(Color::LawnGreen),
            "lemonchiffon" => Ok(Color::LemonChiffon),
            "lightblue" => Ok(Color::LightBlue),
            "lightcoral" => Ok(Color::LightCoral),
            "lightcyan" => Ok(Color::LightCyan),
            "lightgoldenrodyellow" => Ok(Color::LightGoldenRodYellow),
            "lightgray" => Ok(Color::LightGray),
            "lightgrey" => Ok(Color::LightGrey),
            "lightgreen" => Ok(Color::LightGreen),
            "lightpink" => Ok(Color::LightPink),
            "lightsalmon" => Ok(Color::LightSalmon),
            "lightseagreen" => Ok(Color::LightSeaGreen),
            "lightskyblue" => Ok(Color::LightSkyBlue),
            "lightslategray" => Ok(Color::LightSlateGray),
            "lightslategrey" => Ok(Color::LightSlateGrey),
            "lightsteelblue" => Ok(Color::LightSteelBlue),
            "lightyellow" => Ok(Color::LightYellow),
            "lime" => Ok(Color::Lime),
            "limegreen" => Ok(Color::LimeGreen),
            "linen" => Ok(Color::Linen),
            "magenta" => Ok(Color::Magenta),
            "maroon" => Ok(Color::Maroon),
            "mediumaquamarine" => Ok(Color::MediumAquaMarine),
            "mediumblue" => Ok(Color::MediumBlue),
            "mediumorchid" => Ok(Color::MediumOrchid),
            "mediumpurple" => Ok(Color::MediumPurple),
            "mediumseagreen" => Ok(Color::MediumSeaGreen),
            "mediumslateblue" => Ok(Color::MediumSlateBlue),
            "mediumspringgreen" => Ok(Color::MediumSpringGreen),
            "mediumturquoise" => Ok(Color::MediumTurquoise),
            "mediumvioletred" => Ok(Color::MediumVioletRed),
            "midnightblue" => Ok(Color::MidnightBlue),
            "mintcream" => Ok(Color::MintCream),
            "mistyrose" => Ok(Color::MistyRose),
            "moccasin" => Ok(Color::Moccasin),
            "navajowhite" => Ok(Color::NavajoWhite),
            "navy" => Ok(Color::Navy),
            "oldlace" => Ok(Color::OldLace),
            "olive" => Ok(Color::Olive),
            "olivedrab" => Ok(Color::OliveDrab),
            "orange" => Ok(Color::Orange),
            "orangered" => Ok(Color::OrangeRed),
            "orchid" => Ok(Color::Orchid),
            "palegoldenrod" => Ok(Color::PaleGoldenRod),
            "palegreen" => Ok(Color::PaleGreen),
            "paleturquoise" => Ok(Color::PaleTurquoise),
            "palevioletred" => Ok(Color::PaleVioletRed),
            "papayawhip" => Ok(Color::PapayaWhip),
            "peachpuff" => Ok(Color::PeachPuff),
            "peru" => Ok(Color::Peru),
            "pink" => Ok(Color::Pink),
            "plum" => Ok(Color::Plum),
            "powderblue" => Ok(Color::PowderBlue),
            "purple" => Ok(Color::Purple),
            "rebeccapurple" => Ok(Color::RebeccaPurple),
            "red" => Ok(Color::Red),
            "rosybrown" => Ok(Color::RosyBrown),
            "royalblue" => Ok(Color::RoyalBlue),
            "saddlebrown" => Ok(Color::SaddleBrown),
            "salmon" => Ok(Color::Salmon),
            "sandybrown" => Ok(Color::SandyBrown),
            "seagreen" => Ok(Color::SeaGreen),
            "seashell" => Ok(Color::SeaShell),
            "sienna" => Ok(Color::Sienna),
            "silver" => Ok(Color::Silver),
            "skyblue" => Ok(Color::SkyBlue),
            "slateblue" => Ok(Color::SlateBlue),
            "slategray" => Ok(Color::SlateGray),
            "slategrey" => Ok(Color::SlateGrey),
            "snow" => Ok(Color::Snow),
            "springgreen" => Ok(Color::SpringGreen),
            "steelblue" => Ok(Color::SteelBlue),
            "tan" => Ok(Color::Tan),
            "teal" => Ok(Color::Teal),
            "thistle" => Ok(Color::Thistle),
            "tomato" => Ok(Color::Tomato),
            "turquoise" => Ok(Color::Turquoise),
            "violet" => Ok(Color::Violet),
            "wheat" => Ok(Color::Wheat),
            "white" => Ok(Color::White),
            "whitesmoke" => Ok(Color::WhiteSmoke),
            "yellow" => Ok(Color::Yellow),
            "yellowgreen" => Ok(Color::YellowGreen),
            _ => Err("invalid color"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pos {
    line: u32,
    column: u32,
}

impl Pos {
    pub const fn new() -> Self {
        Pos { line: 0, column: 0 }
    }

    pub const fn line(self) -> u32 {
        self.line
    }

    pub const fn column(self) -> u32 {
        self.column
    }

    pub fn newline(&mut self) {
        self.line += 1;
        self.column = 0;
    }

    pub fn next_char(&mut self) {
        self.column += 1;
    }

    pub fn chars(&mut self, num: u32) {
        self.column += num;
    }
}

#[derive(Debug)]
pub enum Property {
    AlignContent,
    AlignItems,
    AlignSelf,
    All,
    Animation,
    AnimationDelay,
    AnimationDirection,
    AnimationDuration,
    AnimationFillMode,
    AnimationIterationCount,
    AnimationName,
    AnimationPlayState,
    AnimationTimingFunction,
    BackfaceVisibility,
    Background,
    BackgroundAttachment,
    BackgroundBlendMode,
    BackgroundClip,
    BackgroundColor,
    BackgroundImage,
    BackgroundOrigin,
    BackgroundPosition,
    BackgroundRepeat,
    BackgroundSize,
    Border,
    BorderBottom,
    BorderBottomColor,
    BorderBottomLeftRadius,
    BorderBottomRightRadius,
    BorderBottomStyle,
    BorderBottomWidth,
    BorderCollapse,
    BorderColor,
    BorderImage,
    BorderImageOutset,
    BorderImageRepeat,
    BorderImageSlice,
    BorderImageSource,
    BorderImageWidth,
    BorderLeft,
    BorderLeftColor,
    BorderLeftStyle,
    BorderLeftWidth,
    BorderRadius,
    BorderRight,
    BorderRightColor,
    BorderRightStyle,
    BorderRightWidth,
    BorderSpacing,
    BorderStyle,
    BorderTop,
    BorderTopColor,
    BorderTopLeftRadius,
    BorderTopRightRadius,
    BorderTopStyle,
    BorderTopWidth,
    BorderWidth,
    Bottom,
    BoxDecorationBreak,
    BoxShadow,
    BoxSizing,
    CaptionSide,
    CaretColor,
    Charset,
    Clear,
    Clip,
    Color,
    ColumnCount,
    ColumnFill,
    ColumnGap,
    ColumnRule,
    ColumnRuleColor,
    ColumnRuleStyle,
    ColumnRuleWidth,
    ColumnSpan,
    ColumnWidth,
    Columns,
    Content,
    CounterIncrement,
    CounterReset,
    Cursor,
    Direction,
    Display,
    EmptyCells,
    Filter,
    Flex,
    FlexBasis,
    FlexDirection,
    FlexFlow,
    FlexGrow,
    FlexShrink,
    FlexWrap,
    Float,
    Font,
    FontFace,
    FontFamily,
    FontKerning,
    FontSize,
    FontSizeAdjust,
    FontStretch,
    FontStyle,
    FontVariant,
    FontWeight,
    Grid,
    GridArea,
    GridAutoColumns,
    GridAutoFlow,
    GridAutoRows,
    GridColumn,
    GridColumnEnd,
    GridColumnGap,
    GridColumnStart,
    GridGap,
    GridRow,
    GridRowEnd,
    GridRowGap,
    GridRowStart,
    GridTemplate,
    GridTemplateAreas,
    GridTemplateColumns,
    GridTemplateRows,
    HangingPunctuation,
    Height,
    Hyphens,
    Import,
    Isolation,
    JustifyContent,
    Keyframes,
    Left,
    LetterSpacing,
    LineHeight,
    ListStyle,
    ListStyleImage,
    ListStylePosition,
    ListStyleType,
    Margin,
    MarginBottom,
    MarginLeft,
    MarginRight,
    MarginTop,
    MaxHeight,
    MaxWidth,
    Media,
    MinHeight,
    MinWidth,
    MixBlendMode,
    ObjectFit,
    ObjectPosition,
    Opacity,
    Order,
    Outline,
    OutlineColor,
    OutlineOffset,
    OutlineStyle,
    OutlineWidth,
    Overflow,
    OverflowX,
    OverflowY,
    Padding,
    PaddingBottom,
    PaddingLeft,
    PaddingRight,
    PaddingTop,
    PageBreakAfter,
    PageBreakBefore,
    PageBreakInside,
    Perspective,
    PerspectiveOrigin,
    PointerEvents,
    Position,
    Quotes,
    Resize,
    Right,
    ScrollBehavior,
    TabSize,
    TableLayout,
    TextAlign,
    TextAlignLast,
    TextDecoration,
    TextDecorationColor,
    TextDecorationLine,
    TextDecorationStyle,
    TextIndent,
    TextJustify,
    TextOverflow,
    TextShadow,
    TextTransform,
    Top,
    Transform,
    TransformOrigin,
    TransformStyle,
    Transition,
    TransitionDelay,
    TransitionDuration,
    TransitionProperty,
    TransitionTimingFunction,
    UnicodeBidi,
    UserSelect,
    VerticalAlign,
    Visibility,
    WhiteSpace,
    Width,
    WordBreak,
    WordSpacing,
    WordWrap,
    WritingMode,
    ZIndex,
}

impl TryFrom<&str> for Property {
    type Error = &'static str;
    fn try_from(string: &str) -> Result<Property, Self::Error> {
        match string {
            "align-content" => Ok(Property::AlignContent),
            "align-items" => Ok(Property::AlignItems),
            "align-self" => Ok(Property::AlignSelf),
            "all" => Ok(Property::All),
            "animation" => Ok(Property::Animation),
            "animation-delay" => Ok(Property::AnimationDelay),
            "animation-direction" => Ok(Property::AnimationDirection),
            "animation-duration" => Ok(Property::AnimationDuration),
            "animation-fill-mode" => Ok(Property::AnimationFillMode),
            "animation-iteration-count" => Ok(Property::AnimationIterationCount),
            "animation-name" => Ok(Property::AnimationName),
            "animation-play-state" => Ok(Property::AnimationPlayState),
            "animation-timing-function" => Ok(Property::AnimationTimingFunction),
            "backface-visibility" => Ok(Property::BackfaceVisibility),
            "background" => Ok(Property::Background),
            "background-attachment" => Ok(Property::BackgroundAttachment),
            "background-blend-mode" => Ok(Property::BackgroundBlendMode),
            "background-clip" => Ok(Property::BackgroundClip),
            "background-color" => Ok(Property::BackgroundColor),
            "background-image" => Ok(Property::BackgroundImage),
            "background-origin" => Ok(Property::BackgroundOrigin),
            "background-position" => Ok(Property::BackgroundPosition),
            "background-repeat" => Ok(Property::BackgroundRepeat),
            "background-size" => Ok(Property::BackgroundSize),
            "border" => Ok(Property::Border),
            "border-bottom" => Ok(Property::BorderBottom),
            "border-bottom-color" => Ok(Property::BorderBottomColor),
            "border-bottom-left-radius" => Ok(Property::BorderBottomLeftRadius),
            "border-bottom-right-radius" => Ok(Property::BorderBottomRightRadius),
            "border-bottom-style" => Ok(Property::BorderBottomStyle),
            "border-bottom-width" => Ok(Property::BorderBottomWidth),
            "border-collapse" => Ok(Property::BorderCollapse),
            "border-color" => Ok(Property::BorderColor),
            "border-image" => Ok(Property::BorderImage),
            "border-image-outset" => Ok(Property::BorderImageOutset),
            "border-image-repeat" => Ok(Property::BorderImageRepeat),
            "border-image-slice" => Ok(Property::BorderImageSlice),
            "border-image-source" => Ok(Property::BorderImageSource),
            "border-image-width" => Ok(Property::BorderImageWidth),
            "border-left" => Ok(Property::BorderLeft),
            "border-left-color" => Ok(Property::BorderLeftColor),
            "border-left-style" => Ok(Property::BorderLeftStyle),
            "border-left-width" => Ok(Property::BorderLeftWidth),
            "border-radius" => Ok(Property::BorderRadius),
            "border-right" => Ok(Property::BorderRight),
            "border-right-color" => Ok(Property::BorderRightColor),
            "border-right-style" => Ok(Property::BorderRightStyle),
            "border-right-width" => Ok(Property::BorderRightWidth),
            "border-spacing" => Ok(Property::BorderSpacing),
            "border-style" => Ok(Property::BorderStyle),
            "border-top" => Ok(Property::BorderTop),
            "border-top-color" => Ok(Property::BorderTopColor),
            "border-top-left-radius" => Ok(Property::BorderTopLeftRadius),
            "border-top-right-radius" => Ok(Property::BorderTopRightRadius),
            "border-top-style" => Ok(Property::BorderTopStyle),
            "border-top-width" => Ok(Property::BorderTopWidth),
            "border-width" => Ok(Property::BorderWidth),
            "bottom" => Ok(Property::Bottom),
            "box-decoration-break" => Ok(Property::BoxDecorationBreak),
            "box-shadow" => Ok(Property::BoxShadow),
            "box-sizing" => Ok(Property::BoxSizing),
            "caption-side" => Ok(Property::CaptionSide),
            "caret-color" => Ok(Property::CaretColor),
            "@charset" => Ok(Property::Charset),
            "clear" => Ok(Property::Clear),
            "clip" => Ok(Property::Clip),
            "color" => Ok(Property::Color),
            "column-count" => Ok(Property::ColumnCount),
            "column-fill" => Ok(Property::ColumnFill),
            "column-gap" => Ok(Property::ColumnGap),
            "column-rule" => Ok(Property::ColumnRule),
            "column-rule-color" => Ok(Property::ColumnRuleColor),
            "column-rule-style" => Ok(Property::ColumnRuleStyle),
            "column-rule-width" => Ok(Property::ColumnRuleWidth),
            "column-span" => Ok(Property::ColumnSpan),
            "column-width" => Ok(Property::ColumnWidth),
            "columns" => Ok(Property::Columns),
            "content" => Ok(Property::Content),
            "counter-increment" => Ok(Property::CounterIncrement),
            "counter-reset" => Ok(Property::CounterReset),
            "cursor" => Ok(Property::Cursor),
            "direction" => Ok(Property::Direction),
            "display" => Ok(Property::Display),
            "empty-cells" => Ok(Property::EmptyCells),
            "filter" => Ok(Property::Filter),
            "flex" => Ok(Property::Flex),
            "flex-basis" => Ok(Property::FlexBasis),
            "flex-direction" => Ok(Property::FlexDirection),
            "flex-flow" => Ok(Property::FlexFlow),
            "flex-grow" => Ok(Property::FlexGrow),
            "flex-shrink" => Ok(Property::FlexShrink),
            "flex-wrap" => Ok(Property::FlexWrap),
            "float" => Ok(Property::Float),
            "font" => Ok(Property::Font),
            "@font-face" => Ok(Property::FontFace),
            "font-family" => Ok(Property::FontFamily),
            "font-kerning" => Ok(Property::FontKerning),
            "font-size" => Ok(Property::FontSize),
            "font-size-adjust" => Ok(Property::FontSizeAdjust),
            "font-stretch" => Ok(Property::FontStretch),
            "font-style" => Ok(Property::FontStyle),
            "font-variant" => Ok(Property::FontVariant),
            "font-weight" => Ok(Property::FontWeight),
            "grid" => Ok(Property::Grid),
            "grid-area" => Ok(Property::GridArea),
            "grid-auto-columns" => Ok(Property::GridAutoColumns),
            "grid-auto-flow" => Ok(Property::GridAutoFlow),
            "grid-auto-rows" => Ok(Property::GridAutoRows),
            "grid-column" => Ok(Property::GridColumn),
            "grid-column-end" => Ok(Property::GridColumnEnd),
            "grid-column-gap" => Ok(Property::GridColumnGap),
            "grid-column-start" => Ok(Property::GridColumnStart),
            "grid-gap" => Ok(Property::GridGap),
            "grid-row" => Ok(Property::GridRow),
            "grid-row-end" => Ok(Property::GridRowEnd),
            "grid-row-gap" => Ok(Property::GridRowGap),
            "grid-row-start" => Ok(Property::GridRowStart),
            "grid-template" => Ok(Property::GridTemplate),
            "grid-template-areas" => Ok(Property::GridTemplateAreas),
            "grid-template-columns" => Ok(Property::GridTemplateColumns),
            "grid-template-rows" => Ok(Property::GridTemplateRows),
            "hanging-punctuation" => Ok(Property::HangingPunctuation),
            "height" => Ok(Property::Height),
            "hyphens" => Ok(Property::Hyphens),
            "@import" => Ok(Property::Import),
            "isolation" => Ok(Property::Isolation),
            "justify-content" => Ok(Property::JustifyContent),
            "@keyframes" => Ok(Property::Keyframes),
            "left" => Ok(Property::Left),
            "letter-spacing" => Ok(Property::LetterSpacing),
            "line-height" => Ok(Property::LineHeight),
            "list-style" => Ok(Property::ListStyle),
            "list-style-image" => Ok(Property::ListStyleImage),
            "list-style-position" => Ok(Property::ListStylePosition),
            "list-style-type" => Ok(Property::ListStyleType),
            "margin" => Ok(Property::Margin),
            "margin-bottom" => Ok(Property::MarginBottom),
            "margin-left" => Ok(Property::MarginLeft),
            "margin-right" => Ok(Property::MarginRight),
            "margin-top" => Ok(Property::MarginTop),
            "max-height" => Ok(Property::MaxHeight),
            "max-width" => Ok(Property::MaxWidth),
            "@media" => Ok(Property::Media),
            "min-height" => Ok(Property::MinHeight),
            "min-width" => Ok(Property::MinWidth),
            "mix-blend-mode" => Ok(Property::MixBlendMode),
            "object-fit" => Ok(Property::ObjectFit),
            "object-position" => Ok(Property::ObjectPosition),
            "opacity" => Ok(Property::Opacity),
            "order" => Ok(Property::Order),
            "outline" => Ok(Property::Outline),
            "outline-color" => Ok(Property::OutlineColor),
            "outline-offset" => Ok(Property::OutlineOffset),
            "outline-style" => Ok(Property::OutlineStyle),
            "outline-width" => Ok(Property::OutlineWidth),
            "overflow" => Ok(Property::Overflow),
            "overflow-x" => Ok(Property::OverflowX),
            "overflow-y" => Ok(Property::OverflowY),
            "padding" => Ok(Property::Padding),
            "padding-bottom" => Ok(Property::PaddingBottom),
            "padding-left" => Ok(Property::PaddingLeft),
            "padding-right" => Ok(Property::PaddingRight),
            "padding-top" => Ok(Property::PaddingTop),
            "page-break-after" => Ok(Property::PageBreakAfter),
            "page-break-before" => Ok(Property::PageBreakBefore),
            "page-break-inside" => Ok(Property::PageBreakInside),
            "perspective" => Ok(Property::Perspective),
            "perspective-origin" => Ok(Property::PerspectiveOrigin),
            "pointer-events" => Ok(Property::PointerEvents),
            "position" => Ok(Property::Position),
            "quotes" => Ok(Property::Quotes),
            "resize" => Ok(Property::Resize),
            "right" => Ok(Property::Right),
            "scroll-behavior" => Ok(Property::ScrollBehavior),
            "tab-size" => Ok(Property::TabSize),
            "table-layout" => Ok(Property::TableLayout),
            "text-align" => Ok(Property::TextAlign),
            "text-align-last" => Ok(Property::TextAlignLast),
            "text-decoration" => Ok(Property::TextDecoration),
            "text-decoration-color" => Ok(Property::TextDecorationColor),
            "text-decoration-line" => Ok(Property::TextDecorationLine),
            "text-decoration-style" => Ok(Property::TextDecorationStyle),
            "text-indent" => Ok(Property::TextIndent),
            "text-justify" => Ok(Property::TextJustify),
            "text-overflow" => Ok(Property::TextOverflow),
            "text-shadow" => Ok(Property::TextShadow),
            "text-transform" => Ok(Property::TextTransform),
            "top" => Ok(Property::Top),
            "transform" => Ok(Property::Transform),
            "transform-origin" => Ok(Property::TransformOrigin),
            "transform-style" => Ok(Property::TransformStyle),
            "transition" => Ok(Property::Transition),
            "transition-delay" => Ok(Property::TransitionDelay),
            "transition-duration" => Ok(Property::TransitionDuration),
            "transition-property" => Ok(Property::TransitionProperty),
            "transition-timing-function" => Ok(Property::TransitionTimingFunction),
            "unicode-bidi" => Ok(Property::UnicodeBidi),
            "user-select" => Ok(Property::UserSelect),
            "vertical-align" => Ok(Property::VerticalAlign),
            "visibility" => Ok(Property::Visibility),
            "white-space" => Ok(Property::WhiteSpace),
            "width" => Ok(Property::Width),
            "word-break" => Ok(Property::WordBreak),
            "word-spacing" => Ok(Property::WordSpacing),
            "word-wrap" => Ok(Property::WordWrap),
            "writing-mode" => Ok(Property::WritingMode),
            "z-index" => Ok(Property::ZIndex),
            _ => Err("invalid property"),
        }
    }
}
