use std::io::Read;

pub fn read_responce<Reader: Read>(mut reader: Reader) -> String {
    let mut buf = [0; 4];
    match reader.read_exact(&mut buf) {
        Ok(_) => {
            let vbuf = buf.to_vec();
            let tmp = String::from_utf8(vbuf).unwrap_or("Encoding error. Use UTF-8.".to_owned());
            if tmp != "F32D" {
                tmp
            } else {
                buf = [0; 4];
                match reader.read_exact(&mut buf) {
                    Ok(_) => {
                        f32::from_be_bytes(buf).to_string()
                    }
                    Err(e) => {
                        println!("{e}");
                        "IOER".to_owned()
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}");
            "IOER".to_owned()
        }
    }
}
