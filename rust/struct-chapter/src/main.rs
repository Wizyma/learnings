mod structs;

fn main() {
    let user = build_user(String::from("toto@gmail.com"), String::from("toto"));
    let user2 = structs::User {
        email: String::from("another@example.com"),
        ..user
    };

    println!("{}", user2.email);
    let scale = 2;
    let rect = structs::Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = structs::Rectangle {
        width: 10,
        height: 5,
    };
    println!("rect is {:#?}", rect);
    dbg!(&rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.calculate_area()
    );

    println!("the rectangle has a nonzero width; is is {}", rect.width());
    println!("the rect can hold rect2 ? {}", rect.can_hold(&rect2));
}

fn build_user(email: String, username: String) -> structs::User {
    structs::User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}



