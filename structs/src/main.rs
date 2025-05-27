fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    impl User {
        fn username_chars_count(&self) -> usize {
            self.username.chars().count()
        }

        fn change_username(&mut self, new_username: String) -> &mut Self {
            self.username = new_username;
            self
        }

        fn deactivate(&mut self) -> &mut Self {
            self.active = false;
            self
        }
    }

    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        username: String::from("user3"),
        email: String::from("user2@example.com"),
        sign_in_count: 2,
        active: true,
    };

    dbg!(&user1);
    println!("Username chars count: {}", &user1.username_chars_count());

    user1.change_username(String::from("user2")).deactivate();
    dbg!(&user1);

    let user3 = User {
        username: String::from("user4"),
        email: String::from("misterbigpenis@example.com"),
        ..user2
    };

    dbg!(&user3);
}
