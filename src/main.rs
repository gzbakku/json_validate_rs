

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
        "name":object! {"type":"string","min":3,"max":6,"errors":object! {
            "min":"",
            "max":""
        }},
        "email":{type:"email",min:3,max:20},
        "age":object! {"type":"number","min":18,max:112,"errors":object! {
            "min":"",
            "max":""
        },elective:true},
        "features": object! {type:"array",min:3,max:5,options:["one","two","three"]},
        "games": object! {type:"object",min:1,max:5,validate: object! {
            dynamic:false,
            schema:object!{
                "cricket":{type:"object",min:2,max:2,validate:object!{
                    schema:{
                        "score":{type:"number",min:1,max:10,options:["2"]},
                        "city":{type:"string",min:2,max:10}
                    }
                }}
            }
        }}
    };

    let data = object! {
        "name":"akku",
        // "age":27,
        "email":"gzbakku@gmail.com",
        "features":["one","two","three"],
        "games":{
            "cricket":{score:2,city:"delhi"},
        }
    };

    let run = validate(
        &format,
        &data,
        "static",
        4
    );

    println!("run : {:?}",run);

}
