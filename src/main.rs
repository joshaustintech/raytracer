use std::io;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use crate::renderer::render;

mod renderer;

fn get_input(message: &'static str) -> bool {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parsed = input.trim();
    return match parsed {
        "y" | "" => true,
        "n" => false,
        _ => {
            println!("Invalid input, defaulting to 'y'");
            true
        }
    }
}

fn main() {
    println!("Rendering...");
    let rendered = render(720);
    if get_input("Write to file? (y/n)\tdefault: 'y'") {
        println!("Writing to file...");
        let mut filename = String::new();
        filename.push_str("render_");
        let unix_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        filename.push_str(&unix_time.to_string());
        filename.push_str(".ppm");
        let mut rendered_file = File::create(filename).expect("Failed to create file");
        rendered_file.write_all(rendered.as_bytes()).expect("Failed to write to file");
        println!("Done!");
    } else {
        println!("{}", rendered);
    }

}
