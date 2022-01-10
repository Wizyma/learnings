fn main() {
    println!("Hello, world!");
    let home = IpAddr {
        kind: IpAdrrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAdrrKind::V6,
        address: String::from("::1"),
    };

    let concise_home = ConciseIpAdrr::V4(127, 0, 0, 1);
    let concise_loopback = ConciseIpAdrr::V6(String::from("::1"));

    use_message()
}

enum  IpAdrrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAdrrKind,
    address: String,
}

enum ConciseIpAdrr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("something")
    }
}

fn use_message() {
    let message = Message::Write(String::from("hello"));
    message.call();
}
