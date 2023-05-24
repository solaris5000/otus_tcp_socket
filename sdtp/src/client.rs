use std::io::Read;

pub fn read_responce<Reader: Read>(mut reader: Reader) -> String {
    let mut buf = [0; 4];
    let responce : (bool, String) = match reader.read_exact(&mut buf) {
        Ok(_) => {
            let vbuf = buf.to_vec();
            let tmp = String::from_utf8(vbuf).unwrap_or("Encoding error. Use UTF-8.".to_owned());
            if tmp != "F32D" {
                (false, tmp)
            } else {
                buf = [0; 4];
                match reader.read_exact(&mut buf) {
                    Ok(_) => {
                        (true, f32::from_be_bytes(buf).to_string())
                    }
                    Err(e) => {
                        println!("{e}");
                        (false, "IOER".to_owned())
                    }
                }
            }
        }
        Err(e) => {
            println!("{e}");
            (false,"IOER".to_owned())
        }
    };

    if responce.0 {
        format!("Current power is: {}", responce.1)
    } else {
        match &responce.1[..] {
            "enbl" => {format!("Socket enabled")},
            "dsbl" => {format!("Socket disabled")},
            "ebld" => {format!("Current state: Enabled")}
            "dbld" => {format!("Current state: Disabled")}
            "E_WC" => {format!("Error: Wrong command")},
            "IORE" => {format!("Error: some I/O error")},
            _ => {format!("Something went wrong while reading responce")},
        }
    }
}
