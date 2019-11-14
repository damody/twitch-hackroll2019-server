
use twitch_api;
use twitch_api::users;

fn main() {
    let mut c = twitch_api::new(String::from("x300i7s4hw2gytbs21h8by9l32wuyf"));
    c.set_oauth_token("3i6w1dn1eeq0cmpjh2fsnihnj432op");

    if let Some(user) = match users::get(&c) {
    Ok(r)  => { println!("{:?}", r); assert!(r.email.is_some()); Some(r) },
    Err(r) => { println!("{:?}", r); assert!(false); None }
    } {
    let user_id = user.id.to_string();

    match users::get_by_id(&c, &user_id) {
        Ok(r)  => {
            assert_eq!(r.name, user.name);
             println!("{:?}", r);
            },
        Err(r) => { println!("{:?}", r); assert!(false); }
    }
}
}
