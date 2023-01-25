use std::io;

fn console() {
    println!("\nLets have fun with BITCOIN\n");
    println!("Do you want to send or receive BITCOIN?\n");

    let mut command = String::new();

    io:stdin().read_line(&mut command);
}

fn main() {
    console()
}
