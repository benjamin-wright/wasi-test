use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <message> <interval>", program);
        return;
    }

    let duration = std::time::Duration::from_secs(args[2].parse().unwrap());

    loop {
        println!("{}", args[1]);
        std::thread::sleep(duration);
    }
}
