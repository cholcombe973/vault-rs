extern crate vault;

fn main() {
    let hosts = vec!["http://localhost:8200"];
    let token = "test12345";
    let client = vault::Client::new(hosts, token).unwrap();

    let _ = client.set_secret("foo", "bar");

    let secret = client.get_secret("foo").unwrap();

    println!("Secret is \"bar\": {}", secret);
}
