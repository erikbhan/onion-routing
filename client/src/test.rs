
use super::*;

#[test]
fn parse_array_test() {
    assert_eq!(parse_array("\\node: 1, key: 1\\".to_string()), (vec!["1".to_string()],vec!["1".to_string()]))
}

#[test]
fn parse_node_test() {
    assert_eq!(parse_node("node: 1, key: 1"), ("1".to_string(),"1".to_string()))
}

// This test passes if the user writes great, but needs to be ignored for ci/cd
#[test]
#[ignore = "Testing user-input from terminal requires adding lots of unnecessary code to support mocking"]
fn get_user_input_test() {
    assert_eq!(get_user_input("How are you? (great)"), "great");
}
/*
#[test]
fn encrypt_message_test() {
    let keys_unformatted = ["Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string()];
    let keys = format_keys(keys_unformatted.to_vec());
    let mut key_copy = keys.to_owned();

    let encrypted = encrypt_message("plaintext".to_string(), keys).expect("This test fails if the method panics");

    key_copy.reverse();

    let key1 = Key::from_slice(&key_copy[0]);
    let key2 = Key::from_slice(&key_copy[1]);
    let key3 = Key::from_slice(&key_copy[2]);

    let cipher1 = Aes256Gcm::new(key1);
    let cipher2 = Aes256Gcm::new(key2);
    let cipher3 = Aes256Gcm::new(key3);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let ciphertext = cipher1.encrypt(nonce, "plaintext".as_bytes())
        .unwrap();
    let ciphertext = cipher2.encrypt(nonce, ciphertext.as_ref())
        .unwrap();
    let ciphertext = cipher3.encrypt(nonce, ciphertext.as_ref())
        .unwrap();
    
    assert_eq!(ciphertext, encrypted)
}

#[test]
fn decrypt_message_test() {
    let keys_unformatted = ["Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string(), "Dette er en kul nokkel som virke".to_string()];
    let keys = format_keys(keys_unformatted.to_vec());
    let key_copy = keys.to_owned();
    let key_copy2 = keys.to_owned();
    let encrypted = encrypt_message("plaintext".to_string(), keys).expect("This test fails if the method panics");
    let encrypted_copy = encrypted.to_owned();

    let decrypted = decrypt_message(encrypted, key_copy2).expect("This test fails if the method panics");

    let key1 = Key::from_slice(&key_copy[0]);
    let key2 = Key::from_slice(&key_copy[1]);
    let key3 = Key::from_slice(&key_copy[2]);

    let cipher1 = Aes256Gcm::new(key1);
    let cipher2 = Aes256Gcm::new(key2);
    let cipher3 = Aes256Gcm::new(key3);

    let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message

    let ciphertext = cipher1.decrypt(nonce, encrypted_copy.as_ref())
    .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
    let ciphertext = cipher2.decrypt(nonce, ciphertext.as_ref())
    .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
    let plaintext = cipher3.decrypt(nonce, ciphertext.as_ref())
    .expect("decryption failure!"); // NOTE: handle this error to avoid panics!
    
    assert_eq!(plaintext, decrypted)
}
*/
#[test]
fn format_keys_test() {
    let keys_unformatted = ["Writing tests is slow and boring".to_string(), "Writing tests is slow and boring".to_string(), "Writing tests is slow and boring".to_string()];
    let keys_formatted = format_keys(keys_unformatted.to_vec());
    let keys = [b"Writing tests is slow and boring".to_owned(), b"Writing tests is slow and boring".to_owned(), b"Writing tests is slow and boring".to_owned()].to_vec();

    assert_eq!(keys_formatted, keys)
}

#[test]
fn key_from_string_test() {
    let key_string = "Writing tests is slow and boring";
    let key = b"Writing tests is slow and boring";
    let key_from_string = key_from_string(key_string.to_string());

    assert_eq!(key.to_owned(), key_from_string)
}