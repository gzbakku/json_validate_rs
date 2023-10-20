

use json::object;

mod validater;
mod common;

// use regex::{Regex,RegexBuilder};

pub use validater::run as validate;

fn main(){

    if false{
        validater::validate_email("g@l.c");
    }

    if true{
        run();
    }

}

fn run() {

    let format = object! {
        "name":object! {
            "type":"array",
            "options":["akku","nikku"],
            "validate":{
                "children_type":"object",
                "schema":{
                    "name":{type:"string"}
                }
            }
        },
    };

    let data = object! {
        "name":[
            {name:19}
        ]
    };

    let run = validate(
        &format,
        &data,
        "dynamic",
        5
    );

    println!("run : {:?}",run);

}
