use std::fs;
fn main() {
    let input_common = fs::read_to_string("input_common.txt").unwrap();
    #[allow(unused)]
    let input = fs::read_to_string("input.txt").unwrap();

    println!("{}", &input_common);

    println!("Board 1");

    let mut lines = input_common.lines();
    let generated = lines.nth(0).unwrap();
    let lines = lines.skip(1);
    println!("genrated: {}", &generated);

    println!("\nRest");
    for i in lines {
        println!("{}", &i);
    }
}

#[allow(unused)]
struct Board {
    line_1: Vec<u32>,
    line_2: Vec<u32>,
    line_3: Vec<u32>,
    line_4: Vec<u32>,
    line_5: Vec<u32>,
}

impl Board {
    #[allow(unused)]
    fn from(board_str: String) -> Self {
        Self {
            line_1: Vec::new(),
            line_2: Vec::new(),
            line_3: Vec::new(),
            line_4: Vec::new(),
            line_5: Vec::new(),
        }
    }
}
