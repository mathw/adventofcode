mod keypadgeometry;
mod movet;
mod parser;

use self::keypadgeometry::Keypad;

pub fn do_day2(input: &str) {
    let instructions =
        input.lines().map(|line| parser::instructions_from_string(line)).collect::<Vec<_>>();

    let mut square_digits = Vec::new();
    let mut diamond_digits = Vec::new();
    let mut current_position_square = Keypad::new_square();
    let mut current_position_diamond = Keypad::new_diamond();

    for line in instructions.iter() {
        current_position_square = current_position_square.apply_moves(line.into_iter());
        current_position_diamond = current_position_diamond.apply_moves(line.into_iter());
        square_digits.push(current_position_square.position);
        diamond_digits.push(current_position_diamond.position);
    }

    println!("On square keypad {}",
             square_digits.into_iter().collect::<String>());
    println!("On diamond keypad {}",
             diamond_digits.into_iter().collect::<String>());
}
