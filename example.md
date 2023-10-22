```rust

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

        "kinky":{
            "akku":{"age":69}
        },

        "address":{
            "country":"india"
        },

    };

    let run = validate(
        &format,
        &data,
        "dynamic",
        10
    );

    println!("run : {:?}",run);

}

```