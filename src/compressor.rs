

use json::{stringify,JsonValue,parse};
use std::io::prelude::*;
use flate2::Compression;
use flate2::write::{ZlibEncoder,ZlibDecoder};
use base64::{Engine as _, engine::general_purpose};

// use std::mem::size_of;

pub fn compress(v:JsonValue)->Result<String,&'static str>{

    let s = stringify(v);
    if s.len() < 5{
        return Ok(format!("0{}",s));
    }


    let cm = &z_compress(&s)?;
    let as_b64 = to_base_64(cm);

    let c1 = format!(
        "1{}",as_b64
    );

    if c1.len() < s.len()+1{
        return Ok(c1);
    } else {
        return Ok(format!("0{}",s));
    }

}

pub fn decompress(s:&str)->Result<JsonValue,&'static str>{

    if s.len() <= 1{
        return Err("invalid-json");
    }

    let cc = s.chars().nth(0).unwrap();
    let end = s.len()-1;
    let part = &s[1..=end];

    if cc == '0'{
        return Ok(to_json(&part)?);
    }
    
    else if cc == '1'{
        let data = from_base_64(part)?;
        let s = z_decompress(&data)?;
        return Ok(to_json(&s)?);
    }

    Err("invalid-flag")

}

fn to_json(v:&str)->Result<JsonValue,&'static str>{
    Ok(
        parse(v).expect("failed-parse-json")
    )
}

fn z_decompress(v:&[u8])->Result<String,&'static str>{
    let mut writer = Vec::new();
    let mut z = ZlibDecoder::new(writer);
    z.write_all(v).expect("ZlibDecoder-write_all");
    writer = z.finish().expect("ZlibDecoder-finish");
    let s = String::from_utf8(writer).expect("z_decompress-string");
    Ok(s)
}

fn z_compress(v:&str)->Result<Box<Vec<u8>>,&'static str>{
    let comp = Compression::best();
    let writer:Box<Vec<u8>> = Box::new(Vec::new());
    let mut e = Box::new(
        ZlibEncoder::new(
            writer, comp
        )
    );
    match e.write_all(v.as_bytes()){
        Ok(_)=>{},
        Err(_e)=>{
            return Err("ZlibEncoder-write_all");
        }
    }
    match e.finish(){
        Ok(_c)=>{
            Ok(_c.clone())
        },
        Err(_e)=>{
            return Err("ZlibEncoder-finish");
        }
    }
}

fn from_base_64(v:&str)->Result<Vec<u8>,&'static str>{
    Ok(
        general_purpose::STANDARD_NO_PAD.decode(v).expect("failed-decode")
    )
}

fn to_base_64(v:&[u8])->String{
    general_purpose::STANDARD_NO_PAD.encode(v)
}

// use flate2::


#[cfg(test)]
mod tests {

    use json::{object,stringify};
    use crate::compressor::{decompress,to_base_64,z_compress};

    #[test]
    fn test_compress() {

        let build = object!{
            "id": "0001",
            "type": "donut",
            "name": "Cake",
            "ppu": 0.55,
            "batters":
                {
                    "batter":
                        [
                            { "id": "1001", "type": "Regular" },
                            { "id": "1002", "type": "Chocolate" },
                            { "id": "1003", "type": "Blueberry" },
                            { "id": "1004", "type": "Devil's Food" }
                        ]
                },
            "topping":
                [
                    { "id": "5001", "type": "None" },
                    { "id": "5002", "type": "Glazed" },
                    { "id": "5005", "type": "Sugar" },
                    { "id": "5007", "type": "Powdered Sugar" },
                    { "id": "5006", "type": "Chocolate with Sprinkles" },
                    { "id": "5003", "type": "Chocolate" },
                    { "id": "5004", "type": "Maple" }
                ]
        };

        let s = stringify(build);

        let compressed_base64 = format!(
            "2{}",
            to_base_64(
                &z_compress(&s).unwrap()
            )
        );

        println!("compressed_base64 : {:?}",compressed_base64.len());

        let d_compressed_base64 = decompress(&compressed_base64);

        println!("d_compressed_base64 : {:?}",d_compressed_base64.is_ok());

        let uncompressed_string = format!("0{}",s);

        println!("uncompressed_string : {:?}",uncompressed_string.len());

        let d_uncompressed_string = decompress(&uncompressed_string);

        println!("d_uncompressed_string : {:?}",d_uncompressed_string.is_ok());

    }
}