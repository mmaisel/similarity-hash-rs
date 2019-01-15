use std::fmt;

#[derive(Debug, Clone)]
pub struct Signature {
    pub blocksize: u32,
    pub signature_1: Vec<u8>,
    pub signature_2: Vec<u8>,
}

impl Signature {
    pub fn from_string(s: &str) -> Signature {
        let components: Vec<&str> = s.split(":").collect();

        if components.len() != 3 {
            // bad digest format
            return Signature::null();
        }

        let blocksize = match components[0].parse::<u32>() {
            Ok(i) => i,
            Err(_) => {
                return Signature::null();
            }
        };

        Signature {
            blocksize: blocksize,
            signature_1: components[1].as_bytes().to_vec(),
            signature_2: components[2].as_bytes().to_vec(),
        }
    }

    pub fn null() -> Signature {
        Signature {
            blocksize: 0,
            signature_1: vec![0],
            signature_2: vec![0],
        }
    }
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.blocksize,
            String::from_utf8(self.signature_1.to_vec()).unwrap_or("0".to_string()),
            String::from_utf8(self.signature_2.to_vec()).unwrap_or("0".to_string()),
        )
    }
}
