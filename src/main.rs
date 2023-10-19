

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
        "action":object! {
            "type":"string",
            "options":["name","age"],
            "option_required_fields":{
                "name":["name","age"]
            },
            "required_fields":["cola"]
        },
        "name":{type:"any",elective:true},
        "age":{type:"any",elective:true},
        "cola":{type:"any",elective:true},
    };

    let data = object! {
        "action":"name",
        // "age":27,
        "name":"akku",
        "cola":"black"
    };

    let run = validate(
        &format,
        &data,
        "dynamic",
        5
    );

    println!("run : {:?}",run);

}
