
fn main() {
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yellow-eyed penguin, 65
    Fiordland penguin, 60
    Invalid, data
    ";

    show_lines(penguin_data);
}

fn show_lines(penguin_data: &str) {
    for (i, line) in penguin_data.lines().enumerate() {
        if i == 0 || line.trim().len() == 0 { continue; }

        let fields = line.split(',')
                                    .map(|field| field.trim())
                                    .collect::<Vec<_>>();
        
        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", line, fields);
        }

        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", fields[0], length);
        }
    }
}
