

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
        check();
    }

}

fn check() {

    let format = object! {

        //-----------------------------
        //simple validations
        //-----------------------------

        //valid data types
        //string number bool object array

        //dynamic validation allows undefined fields
        //static validation only allows defined fields

        //string
        "name":{
            type:"string",
            //should be a valid u64 int
            //checks string length
            //can be applied on all data types
            //only number type impliment dynamic min max int values
            "min":2,"max":256
        },

        //bool
        "member":{
            type:"bool",
            //required_fields prop will check if required keys are present even if they are elective
            "required_fields":["interest"],
            //if elective is set to true and this field is missing check will succeed
            "elective":true
        },

        //numbers
        //valid int types
        // u8, u16, u32, u64
        // i8, i16, i32, i64
        // f64
        "u64":{type:"number",min:1,max:10,int:"u64"},
        "i64":{type:"number",min:-10,max:10,int:"i64"},
        "f64":{type:"number",min:1.0,max:10.0,int:"f64"},

        //-----------------------------
        //array validations
        //-----------------------------

        "interest":{
            type:"string",
            //only allowed string values
            "options":["games","exercise","music"],
            //option required keys
            //can be none for some keys too
            //like music does not require any keys
            "option_required_fields":{
                "games":["games"],
                "exercise":["exercise"]
            },
            "elective":true
        },

        "games":object! {
            "type":"array",
            "validate":{
                //validate array child type
                //can check string,number,bool,object,array
                "children_type":"object",
                //child schema will only be checked if children_type is set to object
                "children_schema":{
                    "name":{type:"string"}
                },
                //check if key for each child object will be unique
                "unique_keys":[
                    "name"
                ]
            },
            //if this item if missing then dont fail
            "elective":true
        },

        "exercise":object! {
            "type":"array",
            //min max checks number of items in a array
            //both min max should be a valid u64 int
            "min":1,"max":10,
            //only these strings will be allowed if set
            "options":["run","swim","gym"],
            "validate":{
                "children_type":"string",
                //check each string value of array is unique
                "unique":true
            },
            "elective":true
        },

        "foods":object! {
            "type":"array",
            "min":1,"max":10,
            //only these strings will be allowed if set
            "validate":{
                "children_type":"object",
                //nested object validation
                "validate_nested_object":{
                    "children_type":"number"
                }
            },
            "elective":true
        },

        //-----------------------------
        //object validations
        //-----------------------------
        
        "kinky":object! {
            "type":"object",
            "validate":{
                "children_type":"object",
                //check child schema if child type is object
                "children_schema":{
                    "age":{type:"number",int:"u16"}
                }
            }
        },

        "address":object! {
            "type":"object",
            "validate":{
                //validate this object schema
                "schema":{
                    "country":{"type":"string"}
                },
            }
        },

        "movie_reviews":object! {
            "type":"object",
            "validate":{
                "children_type":"array",
                "array_validate":{
                    "array_child_type":"object",
                    "validate":{
                        "schema":{
                            "review":{"type":"string"}
                        },
                    }
                }
            },
            "elective":true
        },

        //validate nested object
        //can do schema and children validation on nested objects
        //can also do nested object validation on array object type children
        "deep_object_validate_one":object! {
            "type":"object",
            "validate":{
                "children_type":"object",
                "validate_nested_object":{
                    "validate_nested_object":{
                        "validate_nested_object":{
                            "schema":{
                                "deep":{type:"string"}
                            }
                        }
                    }
                }
            },
            "elective":true
        },

        "deep_object_validate_two":object! {
            "type":"object",
            "validate":{
                "children_type":"object",
                "validate_nested_object":{
                    "validate_nested_object":{
                        "validate_nested_object":{
                            "children_type":"object",
                            "children_schema":{
                                "name":{type:"string"}
                            }
                        }
                    }
                }
            },
            "elective":true
        }

    };

    let data = object! {
        
        //string
        "name":"akku",

        //bool
        "member":true,

        //numbers
        "u64":5,
        "i64":-5,
        "f64":5.0,

        //can be "games","exercise","music"
        "interest":"games",

        "games":[
            object!{"name":"gta"},
            //this will fail if 2 names are same
            // object!{name:"gta"},
            object!{"name":"flight sim"}
        ],

        "exercise":[
            "run","swim",
            //will fail if another run is present
            // "run"
        ],

        "foods":[
            {"paneer":10}
        ],

        "kinky":{
            "akku":{"age":69}
        },

        "address":{
            "country":"india"
        },

        "movie_reviews":{
            // "king":["akku","gzbakku"]
            // "king":[11,12],
            "king":[
                {"review":"good king"}
            ]
        },

        "deep_object_validate_one":{
            "deep_one":{
                "deep_two":{
                    "deep":"true"
                },
                "deep_three":{
                    "deep":"true"
                }
            },
            "deep_two":{
                "deep_two":{
                    "deep":"true"
                },
                "deep_three":{
                    "deep":"true"
                }
            }
        },

        "deep_object_validate_two":{
            "deep_one":{
                "deep_two":{
                    "deep_three":{
                        "name":"true"
                    }
                },
                "deep_three":{
                    "deep_three":{
                        "name":"true"
                    }
                }
            },
            "deep_two":{
                "deep_two":{
                    "deep_three":{
                        "name":"true"
                    }
                },
                "deep_three":{
                    "deep_three":{
                        "name":"true"
                    }
                }
            }
        }

    };

    let run = validate(
        &format,
        &data,
        "dynamic",
        14
    );

    println!("run : {:?}",run);

}
