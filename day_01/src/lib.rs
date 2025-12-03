
pub fn get_password_from_input(input: &str, start: isize, size: isize) -> isize {
    let mut current = start;

    let mut count = 0;
    for line in input.lines() {
        let indicator = &line[..1];
        let is_right = indicator == "R";
        let move_amount = line[1..].parse::<isize>().unwrap() % size;

        current = if is_right { current + move_amount } else { current - move_amount };

        // println!("{}, {}, {}", indicator, is_right, move_amount);
        // println!("{} = {}", line, current);

        if current % size == 0 {
            count += 1;
        }
    }

    count
}

pub fn get_new_password_from_input(input: &str, start: isize, size: isize) -> isize {
    let mut current = start;

    let mut count = 0;
    for line in input.lines() {
        let sign = match &line[..1] {
            "L" => -1,
            "R" => 1,
            _ => panic!("Invalid direction"),
        };
        let move_amount = line[1..].parse::<isize>().unwrap();

        let rounds = move_amount / 100;
        let move_amount = move_amount % 100;

        let new_pos = (current + (move_amount * sign) + size) % size;

        let previous = current;
        current = new_pos;
        count += rounds;

        if previous != 0 && ((sign > 0 && new_pos < previous) || (sign < 0 && new_pos > previous) || new_pos == 0) {
            count += 1;
        }
    }

    count
}
