

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
                },
                "unique_keys":[
                    "name"
                ]
            }
        },
        "games":object! {
            "type":"array",
            "validate":{
                "children_type":"string",
                "unique":true
            }
        },
    };

    let data = object! {
        "name":[
            // "akku"
            {name:"akku"},
            // {name:"akku"},
            {name:"nikku"},
        ],
        "games":[
            "gta","gta.1"
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
