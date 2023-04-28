# json_validate_rs
## Json validator in rust

this is a simple schema validator for json in rust

## supported data types

- Import a HTML file and watch it magically convert to Markdown
- Drag and drop images (requires your Dropbox account be linked)
- Import and save files from GitHub, Dropbox, Google Drive and One Drive
- Drag and drop markdown and HTML files into Dillinger
- Export documents as Markdown, HTML and PDF

## supported validation types

- String = min,max,options
- number = min,max,options
- array = min,max
- object = min,max,schema

```rust
use json_validate_rs;

use json_validate_rs::validate;

fn main() {

    let format = object! {
        "name":object! {"type":"string","min":3,"max":6,"errors":object! {
            "min":"",
            "max":""
        }},
        "age":object! {"type":"number","min":18,max:112,"errors":object! {
            "min":"",
            "max":""
        }},
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
        "age":27,
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

```
