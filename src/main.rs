

use json::object;

pub mod validater;
pub mod common;
pub mod compressor; 

// use regex::{Regex,RegexBuilder};

pub use validater::run as validate;

fn main(){

    if false{
        let _ = validater::validate_email("g@l.c");
    }

    if true{
        check();
    }

    if false{
        compression_test();
    }

}

fn compression_test(){

    let map = object!{
        "vars":{},
        "blocks":{
            "386bba5a5dc4fac215c9cf0b9a29b352":{
                id:"386bba5a5dc4fac215c9cf0b9a29b352",
                actions:{}
            },
        }
    };

    let cc = compressor::compress(map).unwrap();

    println!("cc : {cc}");

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

        //definitions
        //these definitions can be used in place of key rules,children_schema and schema objects
        "$_DEFINE_$":{
            name:{type:"string",min:1,max:100,options:["akku"]},
            age:{type:"number",min:1,max:100},
            new:{type:"bool"},
            person_schema:{
                name:"name",
                age:"age",
                new:"new"
            },
            group:{type:"array",min:1,max:10,validate:{
                children_type:"object",
                children_schema:"person_schema"
            }},
            class:{type:"object",min:1,max:1,validate:{
                schema:{
                    group:"group"
                }
            }},
            school:{type:"object",min:1,max:10,validate:{
                //only these keys will be allowed in a object
                valid_keys:["one","two","three"],
                children_schema:{
                    class:"class"
                }
            }},
        },

        "class":"class",

        "school":"school",

        //include exclude and include_any keys 
        //these will check fellow keys in object if self is present
        //include any will trigger if all keys are missing
        "engine":{type:"bool","include_any":["plane","car"],"else":["maggie"],"elective":true},
        "plane":{type:"bool","include":["pilot","engine"],"exclude":["car"],"elective":true},
        "car":{type:"bool","include":["driver","engine"],"exclude":["pilot"],"elective":true},
        "pilot":{type:"bool","include":["plane"],"exclude":["car"],"elective":true},
        "driver":{type:"bool","include":["car"],"exclude":["plane"],"elective":true},

        "maggie":{type:"bool","elective":true},

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
                "unique":true,
                //checks min string len
                "min_string":3,
                //checks max string len
                "max_string":10
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
                    //checks min key len
                    "min_key_size":1,
                    //checks min key len
                    "max_key_size":10,
                    //checks child data type
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

        "class":{
            "group":[
                {"name":"akku","age":26,"new":false}
            ]
        },

        "school":{
            "one":{
                "class":{
                    "group":[
                        {"name":"akku","age":26,"new":false}
                    ]
                }
            },
            "two":{
                "class":{
                    "group":[
                        {"name":"akku","age":26,"new":false}
                    ]
                }
            },
        },

        // "engine":true,
        // "plane":true,
        // "pilot":true,
        // "driver":true,

        "maggie":false,
        
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
        21
    );

    println!("validate : {:?}",run);

    // let c_big = compressor::compress(data).unwrap();

    // println!("c_big : {}",c_big);

    // let c_small = compressor::compress(object!{
    //     name:"akku",akku:"name",age:"99","99":"age"
    // }).unwrap();

    // println!("c_small : {}",c_small);

    // println!("c_big decompress : {:?}",compressor::decompress(&c_big).is_ok());

    // println!("c_small decompress : {:?}",compressor::decompress(&c_small).is_ok());

    // let mut _c_test = object!{};
    // for i in 1..10000{
    //     _c_test[i.to_string()] = format!("sa987das987das98d7as987d7d {i}").into();
    // }
    // let c_test = compressor::compress(_c_test).unwrap();
    // println!("c_test decompress : {:?}",compressor::decompress(&c_test).is_ok());

    

}
