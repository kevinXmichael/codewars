// mod test;
mod encrypt_this;

fn main() {
    // test::run();

    // let result = encrypt_this::encrypt_this("test me");
    let result = encrypt_this::encrypt_this("test me");
    println!("The result is: '{}'", result);
}
