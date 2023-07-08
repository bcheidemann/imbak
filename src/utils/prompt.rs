use anyhow::Context;

pub fn prompt<S: AsRef<str>>(prompt: S, password: bool) -> anyhow::Result<String> {
    let mut input = String::new();
    println!("{}: ", prompt.as_ref());

    if password {
        rpassword::read_password().context("Failed to read password")
    } else {
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read line")?;
        Ok(input)
    }
}
