use rustc_serialize::hex::ToHex;
use md5;

const PASSCODE_LENGTH: usize = 8;

pub fn do_dayfive() {
    let key = "abbhdwsy".to_owned();

    let mut found_positions = Vec::new();

    let digits = (0..u64::max_value()).into_iter().filter_map(|index| {
        let input = key.clone() + &index.to_string();
        let output = md5::compute(input.as_bytes());

        // check the first two bytes in binary to avoid most string conversions
        if output[0] == 0 && output[1] == 0 {
            let hex = output.to_hex();
            if hex.starts_with("00000") {
                let mut interesting = hex.chars().skip(5);
                let pos = usize::from_str_radix(&interesting.next().unwrap().to_string(), 16)
                    .unwrap();
                let digit = interesting.next().unwrap();

                println!("{} {} {}", hex, pos, digit);
                if pos >= PASSCODE_LENGTH {
                    return None;
                } else {
                    if found_positions.contains(&pos) {
                        return None;
                    } else {
                        found_positions.push(pos.clone());
                        return Some((pos, digit));
                    }
                }
            }
        }
        None
    });

    let mut result = [0 as char; PASSCODE_LENGTH];
    let mut seenpos = Vec::new();

    for (pos, digit) in digits {
        println!("Processing position {} digit is {}", pos, digit);
        if seenpos.contains(&pos) {
            continue;
        }

        seenpos.push(pos);
        result[pos] = digit;

        if seenpos.len() >= PASSCODE_LENGTH {
            break;
        }
    }

    println!("Found digits {:?}",
             result.iter().cloned().collect::<String>());
}
