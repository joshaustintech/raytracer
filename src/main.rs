use std::io;
use std::fs::File;
use std::io::Write;
use std::time::SystemTime;
use crate::renderer::vector3d::Color;

mod renderer;

fn render(width: u32, height: u32) -> String {
    println!("Width: {}px, Height: {}px", width, height);
    let mut render = String::new();
    render.push_str("P3\n");
    render.push_str(&width.to_string());
    render.push_str(" ");
    render.push_str(&height.to_string());
    render.push_str("\n255\n");
    for y in (0..height).rev() {
        for x in 0..width {
            let color = Color {
                x: x as f64 / width as f64,
                y: y as f64 / height as f64,
                z: 0.45
            };
            render.push_str(color.get_color().as_str());
            render.push_str("\n");
        }

        if y % (height / 4) == 0 {
            println!("{}% complete", 100 - (y * 100 / height));
        }
    }
    return render;
}

fn get_input(message: &'static str) -> bool {
    println!("{}", message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let parsed = input.trim();
    return if parsed == "y" || parsed.is_empty() {
        true
    } else if parsed == "n" {
        false
    } else {
        println!("Invalid input, defaulting to 'y'");
        true
    }
}

fn main() {
    println!("Rendering...");
    let rendered = render(512, 512);
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
