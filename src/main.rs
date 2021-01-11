fn write_ppm(width: i32, height: i32) {
    println!("Writing {} x {} ppm", width, height);
}

fn main() {
    let width = 256i32;
    let height = 256i32;

    write_ppm(width, height);
}
