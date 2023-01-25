use std::io;
use rand::Rng;


fn send_bitcoin() {
    println!("Hello");


}

fn receive_bitcoin() {
    println!("\nWe're going to receive some Bitcoin!");

    let amount = rand::thread_rng().gen_range(1..=10);

    println!("You've just received {} \n", amount);
    
}

fn exit_console() {
    println!("Invalid option, must be (s) or receive (r)");

}

fn console() {
    println!("\nLets have fun with Bitcoin\n");
    println!("Do you want to send or receive Bitcoin?\n");

    let mut command = String::new();

    io::stdin().read_line(&mut command);

    if command.trim().eq("s"){
        send_bitcoin();
    }
    else if command.trim().eq("r")
    {
        receive_bitcoin();
    }
}

fn main() {
    console()
}
