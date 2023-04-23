
use core::fmt;
use std::str::FromStr;
use ryu;

// Represents the program stack
pub type Stack = Vec<WValue>;

#[derive(Debug, Copy, Clone)]            // For printing out the OpBinary, for example Add.
pub enum OpBinary {
    Add,
    Subtract,
    Multiply,
    FDivide,
    IDivide,
    RGreater,
    LGreater,
    Equality,
    And,
    Or 
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]  // PartialEq so we can compare to lists, debug for printing 
pub enum WValue {
    VInt (i64),
    VFloat (f32),
    VBool (bool),
    VString (String),
    VList (Vec<WValue>),
    VCodeBlock (String),   
    VOther (String)
}

pub enum ParseError {
    StackEmpty,
    NotEnoughElements,
    CouldNotParse,
    EmptyOrNotCorrectType,
    MissingClosingQuote,
    ExpectedVOther,
    ExpectedCodeblock,
    ExpectedQuotation,
    ExpectecCodeBlockOrValidOperation,
    ExpectedBoolOrNumber,
    ExpectedString,
    ExpectedList,
    DivisionByZero,
    NonCompatibleTypes,
    FirstElemNotValid,
    InvalidListElement,
    ListEmpty
    // Add any other error variants you need
}

// To display the wrapped types as strings
impl fmt::Display for WValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WValue::VInt(n) => write!(f, "{}", n),
            WValue::VFloat(fl) => {
                
                let mut buf = ryu::Buffer::new(); // Searched online and found the crate ryu converting to
                write!(f, "{}", buf.format(*fl)) // converting to floating points with correct Decimal values
            }
            WValue::VBool(b) => write!(f, "{}", b),
            WValue::VString(s) => write!(f, "{}", s),
            WValue::VList(list) => {   // Vec<WValue> does not implement fmt::Display so need to do it customly
                write!(f, "[")?;
                for (i, value) in list.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            WValue::VCodeBlock(cb) => write!(f, "{}", cb),    
            WValue::VOther(o) => write!(f, "{}", o),
        }
    }
}

// To convert from string to the enum type
impl WValue {
    pub fn from_string(s: &str) -> WValue {
        if let Ok(num) = i64::from_str(s) {
            WValue::VInt(num)
        } else if let Ok(num) = f32::from_str(s) {
            WValue::VFloat(num)
        } else if let Ok(b) = bool::from_str(s) {
            WValue::VBool(b)
        } else if s.starts_with("\"") && s.ends_with("\"") {
            WValue::VString(s.to_string())
        } else {
            WValue::VOther(s.to_string())
        }
    }
} 

