// Copyright 2018 Grove Enterprises LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Debug;
use erased_serde::*;

/// The data types supported by this database. Currently just u64 and string but others
/// will be added later, including complex types
#[derive(Debug,Clone)]
pub enum DataType {
    UnsignedLong,
    String,
    Double,
    ComplexType(Vec<Field>)
}

/// Definition of a column in a relation (data set).
#[derive(Debug,Clone)]
pub struct Field {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool
}

impl Field {
    pub fn new(name: &str, data_type: DataType, nullable: bool) -> Self {
        Field {
            name: name.to_string(),
            data_type: data_type,
            nullable: nullable
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {:?}", self.name, self.data_type)
    }
}

#[derive(Debug,Clone)]
pub struct ComplexType {
    name: String,
    fields: Vec<Field>
}

/// Definition of a relation (data set) consisting of one or more columns.
#[derive(Debug,Clone)]
pub struct Schema {
    pub columns: Vec<Field>
}

impl Schema {

    /// create an empty tuple
    pub fn empty() -> Self { Schema { columns: vec![] } }

    pub fn new(columns: Vec<Field>) -> Self { Schema { columns: columns } }

    /// look up a column by name and return a reference to the column along with it's index
    pub fn column(&self, name: &str) -> Option<(usize, &Field)> {
        self.columns.iter()
            .enumerate()
            .find(|&(_,c)| c.name == name)
    }

    pub fn to_string(&self) -> String {
        let s : Vec<String> = self.columns.iter()
            .map(|c| c.to_string())
            .collect();
        s.join(",")
    }

}

#[derive(Debug,Clone)]
pub struct FunctionMeta {
    pub name: String,
    pub args: Vec<Field>,
    pub return_type: DataType
}

/// A tuple represents one row within a relation and is implemented as a trait to allow for
/// specific implementations for different data sources
//pub trait Tuple {
//    fn get_value(&self, index: usize) -> Result<Value, Box<Error>>;
//}

#[derive(Debug)]
pub struct Row {
    pub values: Vec<Box<Value>> //TODO: should be references
}

impl Row {

    pub fn new(v: Vec<Box<Value>>) -> Self {
        Row { values: v }
    }

    pub fn to_string(&self) -> String {
        let value_strings : Vec<String> = self.values.iter()
            .map(|v| v.to_string())
            .collect();

        // return comma-separated
        value_strings.join(",")
    }
}

/// Marker trait for the values
pub trait Value: Debug {
    //fn add(other: &Value)
    fn to_string(&self) -> String;
}

impl Clone for Box<Value> {
    fn clone(&self) -> Self {
        unimplemented!()
    }

    fn clone_from(&mut self, source: &Self) {
        unimplemented!()
    }
}
//
//impl Value for f64 {}
//impl<'a> Value for Str<'a> {}

/// Value holder for all supported data types
//#[derive(Debug,Clone,PartialEq)]
//pub enum Value {
//    UnsignedLong(u64),
//    String(String),
//    Boolean(bool),
//    Double(f64),
//    ComplexValue(Vec<Value>)
//}

#[derive(Debug, Clone)]
struct ComplexValue {

}

impl Value for u64 {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

impl Value for f64 {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

impl Value for String {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

impl Value for ComplexValue {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

#[derive(Debug,Clone)]
pub enum Operator {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus
}

/// Relation Expression
#[derive(Debug,Clone)]
pub enum Expr {
    /// index into a value within the tuple
    TupleValue(usize),
    /// literal value
    Literal(Box<Value>),
    /// binary expression e.g. "age > 21"
    Binary { left: Box<Expr>, op: Operator, right: Box<Expr> },
    /// sort expression
    Sort { expr: Box<Expr>, asc: bool },
    /// scalar function
    ScalarFunction { name: String, args: Vec<Expr> }
}

impl Expr {

    pub fn eq(&self, other: &Expr) -> Expr {
        Expr::Binary {
            left: Box::new(self.clone()),
            op: Operator::Eq,
            right: Box::new(other.clone())
        }
    }

    pub fn gt(&self, other: &Expr) -> Expr {
        Expr::Binary {
            left: Box::new(self.clone()),
            op: Operator::Gt,
            right: Box::new(other.clone())
        }
    }

    pub fn lt(&self, other: &Expr) -> Expr {
        Expr::Binary {
            left: Box::new(self.clone()),
            op: Operator::Lt,
            right: Box::new(other.clone())
        }
    }

}

/// Relations
#[derive(Debug,Clone)]
pub enum LogicalPlan {
    Limit { limit: usize, input: Box<LogicalPlan>, schema: Schema },
    Projection { expr: Vec<Expr>, input: Box<LogicalPlan>, schema: Schema },
    Selection { expr: Expr, input: Box<LogicalPlan>, schema: Schema },
    Sort { expr: Vec<Expr>, input: Box<LogicalPlan>, schema: Schema },
    TableScan { schema_name: String, table_name: String, schema: Schema },
    CsvFile { filename: String, schema: Schema },
    EmptyRelation
}

impl LogicalPlan {

    pub fn schema(&self) -> Schema {
        match self {
            &LogicalPlan::EmptyRelation => Schema::empty(),
            &LogicalPlan::TableScan { ref schema, .. } => schema.clone(),
            &LogicalPlan::CsvFile { ref schema, .. } => schema.clone(),
            &LogicalPlan::Projection { ref schema, .. } => schema.clone(),
            &LogicalPlan::Selection { ref schema, .. } => schema.clone(),
            &LogicalPlan::Sort { ref schema, .. } => schema.clone(),
            &LogicalPlan::Limit { ref schema, .. } => schema.clone(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::LogicalPlan::*;
    use super::Expr::*;
    extern crate serde_json;

    #[test]
    fn serde() {

        let tt = Schema {
            columns: vec![
                Field { name: "id".to_string(), data_type: DataType::UnsignedLong, nullable: false },
                Field { name: "name".to_string(), data_type: DataType::String, nullable: false }
            ]
        };

        let csv = CsvFile { filename: "test/data/people.csv".to_string(), schema: tt.clone() };

        let filter_expr = Binary {
            left: Box::new(TupleValue(0)),
            op: Operator::Eq,
            right: Box::new(Literal(Box::new(2_u64)))
        };

        let plan = Selection {
            expr: filter_expr,
            input: Box::new(csv),
            schema: tt.clone()

        };

        unimplemented!()

//        let s = serde_json::to_string(&plan).unwrap();
//        println!("serialized: {}", s);
    }

}

