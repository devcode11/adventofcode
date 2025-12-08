pub fn read_input(day: u32, example: bool) -> String {
    let input_file_path = format!(
        "inputs/day{}{}.txt",
        day,
        if example { "_example" } else { "" }
    );
    std::fs::read_to_string(input_file_path).unwrap()
}
