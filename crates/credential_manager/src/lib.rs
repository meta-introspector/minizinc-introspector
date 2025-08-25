use keyring::Entry;
use keyring::Error; // Import Error type

pub fn store_credential(service: &str, username: &str, password: &str) -> Result<(), Error> {
    let entry = Entry::new(service, username)?;
    entry.set_password(password)
}

pub fn retrieve_credential(service: &str, username: &str) -> Result<String, Error> {
    let entry = Entry::new(service, username)?;
    entry.get_password()
}