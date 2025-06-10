use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Scrypt
};

fn main() {

    // Choose any password you want
    let password = b"toomanysecrets";

    // Encrypt the password into a PHC string.
    let password_phc = create_password_phc(password);
    println!("Create: {}", &password_phc);

    // Verify the password matches the PHC string.
    let result = verify_password(password, &password_phc);
    println!("Verify: {}", result.is_ok());

}

// Create a password PHC string.
pub fn create_password_phc(password: &[u8]) -> String {

    // Create a random salt string to increase resistance to attacks
    let salt = SaltString::generate(&mut OsRng);

    // Hash the password to an Scrypt PasswordHash struct.
    let password_hash = Scrypt.hash_password(password, &salt).expect("scrypt.hash_password");

    // Return the password hash formatted as a PHC string ($scrypt$v=19$...)
    password_hash.to_string()

}

// Verify a password against a PHC string.
pub fn verify_password(password: &[u8], password_phc_str: &str) -> Result<(), scrypt::password_hash::Error> {

    // Parse the password PHC string to an Scrypt PasswordHash struct.
    let password_hash = PasswordHash::new(&password_phc_str).expect("PasswordHash::new");

    // Verify the password plain text byte array with the Scrypt PasswordHash struct.
    Scrypt.verify_password(password, &password_hash)

}
