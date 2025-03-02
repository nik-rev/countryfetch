use local_ip_address::linux::local_ip;

fn main() {
    let ip = local_ip().unwrap();
    println!("{ip:?}");
}

unsafe auto trait Lol {}
