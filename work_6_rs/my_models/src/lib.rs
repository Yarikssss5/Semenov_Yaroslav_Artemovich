use serde::{Deserialize, Serialize};

#[cfg(feature = "native")]
pub mod native;

pub trait MyTableDDL {
    const MY_CREATE_TABLE_DDL: &'static str;
}

#[cfg_attr(feature = "native", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyStudent {
    pub id: i64,
    pub first_name: String, // Имя
    pub middle_name: String, // Фамилия
    pub last_name: String, // Отчество
}

impl MyStudent {
    pub fn new(first_name: String, middle_name: String, last_name: String, id: i64) -> Self {
        Self{ first_name, middle_name, last_name, id }
    }
}

impl MyTableDDL for MyStudent {
    const MY_CREATE_TABLE_DDL: &'static str = " CREATE TABLE IF NOT EXISTS t_students (
        id BIGINT PRIMARY KEY,
        first_name TEXT NOT NULL,
        middle_name TEXT NOT NULL,
        last_name TEXT NOT NULL
    )";
}

impl Default for MyStudent {
    fn default() -> Self {
        Self { first_name: Default::default(), middle_name: Default::default(), last_name: Default::default(), 
            id: Default::default() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetStudentsResult {
    pub res: Result<Vec<MyStudent>, String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyCreateStudentResult {
    pub res: Result<MyStudent, String>
}

impl From<Result<Vec<MyStudent>, String>> for GetStudentsResult {
    fn from(value: Result<Vec<MyStudent>, String>) -> Self {
        Self { res: value }
    }
}

impl From<Result<MyStudent, String>> for MyCreateStudentResult {
    fn from(value: Result<MyStudent, String>) -> Self {
        Self { res: value }
    }
}

pub fn my_split_string(str: String) -> Vec<String> {
    let mut buf: String = String::new();
    let mut res: Vec<String> = vec![];
    for mychar in str.chars() {
        if mychar != ' ' {
            buf = buf.to_string() + &mychar.to_string();
        } else {
            res.push(buf.clone());
            buf = String::new();   
        }
    }
    res
}

pub fn my_safe_get_str_from_vec(vec: Vec<String>, i: usize) -> String {
    let buf: Option<&String> = vec.get(i);
    if buf.is_none() {
        String::from("      ")
    } else {
        buf.unwrap().to_string()
    }
}

impl ToString for MyStudent {
    fn to_string(&self) -> String {
        format!("ID: {}, First name : {}, Middle name : {}, Last name : {} ", self.id, self.first_name, self.middle_name, self.last_name)
    }
}

pub struct MyVecString(pub Vec<String>);

impl From<Vec<String>> for MyVecString {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}

// impl From<MyVecString> for

impl ToString for MyVecString {
    fn to_string(&self) -> String {
        let mut buf = String::new();
        for i in self.0.clone() {
            buf = buf + ", \n" + &i;
        } 
        buf
    }
}