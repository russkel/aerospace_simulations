// Lab 01 - Square Numbers

fn main() {
    let mut raw = String::new();

    println!("Input numbers separated by space to be squared:");

    std::io::stdin().read_line(&mut raw).expect("Failed to read STDIN");
    let numbers: Vec<f32> = raw.trim().split(' ').map(|s|
        s.parse::<f32>().unwrap_or(0.0)
            .powf(2.0)).collect();

    println!("{:?}", numbers);
}
