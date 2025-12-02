use std::fs::read_to_string;



fn main() {
    let file_name = "input.txt";
    let mut pointer: i16 = 50;
    let mut password = 0;
    for line in read_to_string(file_name).unwrap().lines() {
        let (direction, number) = (&line[..1], &line[1..]);
        let number: i16 = number.parse().unwrap();
        
        match direction {
            "L" => pointer -= number,
            "R" => pointer += number,
            _ =>  panic!("unexpected direction"),
        }

        pointer = pointer % 100; 
        if pointer == 0 {
            password += 1;
        }

    }

    println!("{password}")
}
