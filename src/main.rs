use base64::{engine::general_purpose, Engine as _};

fn main() {
    let mut encoded = "0XeoBXYyd2b0BXeyN2X09mbgMXafNXaoR3eTJEV".chars().rev().collect::<String>();

    for _ in 0..5 {
        if let Ok(decoded) = general_purpose::STANDARD.decode(encoded.as_bytes()) {
            if let Ok(s) = String::from_utf8(decoded) {
                encoded = s;
            } else {
                break;
            }
        } else {
            break;
        }
    }

    while encoded.len() % 4 != 0 {
        encoded.push('=');
    }

    if let Ok(decoded) = general_purpose::STANDARD.decode(encoded.as_bytes()) {
        if let Ok(result) = String::from_utf8(decoded) {
            println!("decoded: {}", result);
        }
    }
}
