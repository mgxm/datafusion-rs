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

extern crate datafusion;
use datafusion::rel::*;
use datafusion::exec::*;

extern crate serde_json;

/// This example shows the use of the DataFrame API to define a query plan
fn main() {

    // create execution context
    let ctx = ExecutionContext::local("./tests/data".to_string());

    // define schema for data source (csv file)
    let schema = Schema::new(vec![
        Field::new("city", DataType::String, false),
        Field::new("lat", DataType::Double, false),
        Field::new("lng", DataType::Double, false)]);

    // open a CSV file as a dataframe
    let df1 = ctx.load("tests/data/uk_cities.csv", &schema).unwrap();
    println!("df1: {}", df1.schema().to_string());

    // filter on lat > 52.0
    let lat = df1.col("lat").unwrap();
    let value = Expr::Literal(Value::Double(52.0));
    let df2 = df1.filter(lat.gt(&value)).unwrap();
    println!("df2: {}", df1.schema().to_string());

    // apply a projection using a scalar function to create a complex type
    // invoke custom code as a scalar UDF
    let st_point = ctx.udf("ST_Point",vec![
        df2.col("lat").unwrap(),
        df2.col("lng").unwrap()]);

    let df3 = df2.select(vec![st_point]).unwrap();
    println!("df3: {}", df1.schema().to_string());

    // write the results to a file
    ctx.write(df3, "_northern_cities.csv").unwrap();

}
