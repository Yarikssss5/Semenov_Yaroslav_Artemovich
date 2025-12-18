use serde::{Deserialize, Serialize};

#[cfg(feature = "native")]
pub mod native;

// Task 1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct  MyFileParsed {
    pub name: String,
    pub size: i32,
    pub data: String,
    pub mime_type: String
}

// Task 2
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyActionKind {
    Plus,
    Minus
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyAction {
    pub kind: MyActionKind,
    pub value: i64
}

impl PartialEq for MyActionKind {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}


impl PartialEq for MyAction {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}

// Task 3

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Friend {
    pub id: u64,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recommendation {
    pub id: u64,
    pub username: String,
    pub common_count: usize,
}

// Task 4 
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MyTaskPriority {
    Common,
    Emergancy,
    Expired
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MyTask {
    pub str: String,
    pub priority: MyTaskPriority
}

impl ToString for MyTaskPriority {
    fn to_string(&self) -> String {
        match self {
            MyTaskPriority::Common => "Common".to_string(),
            MyTaskPriority::Emergancy => "Emergancy".to_string(),
            MyTaskPriority::Expired => "Expired".to_string(),
        }
    }
}

impl ToString for MyTask {
    fn to_string(&self) -> String {
        format!("{} : {}", self.str, self.priority.to_string())
    }
}

impl From<String> for MyTaskPriority {
    fn from(value: String) -> Self {
        if value == "Common".to_string() {
            Self::Common
        } else if value == "Emergancy".to_string() {
            Self::Emergancy
        } else if value == "Expired".to_string() {
            Self::Expired
        } else {
            Self::Common
        }
    }
}

impl From<f64> for MyTaskPriority {
    fn from(value: f64) -> Self {
        if value == f64::from(0) {
            Self::Common
        } else if value == f64::from(1) {
            Self::Emergancy
        } else if value == f64::from(2) {
            Self::Expired
        } else {
            Self::Common
        }
    }
}

// Task 5 
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MyProduct {
    pub name: String,
    pub cost: u64,
    pub id: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MyProductInCart {
    pub product: MyProduct,
    pub count: u64,
}

#[macro_export]
macro_rules! my_rub {
    ($rubles:expr) => {
        $rubles * 100
    };
}

// Task 6 
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MyNotification {
    pub author: String,
    pub text: String
}

pub struct MyU64(pub u64);

impl From<MyU64> for u64 {
    fn from(value: MyU64) -> Self {
        value.0
    }
}

impl TryFrom<String> for MyU64 {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match u64::from_str_radix(&value, 10) {
            Ok(val) => Ok(Self(val)),
            Err(e) => Err(e.to_string())
        }
    }
}