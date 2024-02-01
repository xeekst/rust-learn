extern crate crypto;

use crypto::{
    aes::{self, KeySize::KeySize256},
    blockmodes::PkcsPadding,
    buffer::{ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer},
    symmetriccipher::SymmetricCipherError,
};

fn main() {
    let data = "1qasw@";
    let encrypted_data = aes_encrypt(data).unwrap();
    println!("aes_encrypt:{:?}", encrypted_data);
    let decrypted_data = aes_decrypt(&encrypted_data).unwrap();
    println!("aes_decrypt:{}", decrypted_data);
}

fn aes_encrypt(data: &str) -> Result<String, SymmetricCipherError> {
    let iv = [7u8; 16];
    let key = [7u8; 32];
    let encrypted_data = aes256_cbc_encrypt(data.as_bytes(), &key, &iv)?;

    let hex_string = encrypted_data
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<String>();

    Ok(hex_string)
}

fn aes_decrypt(data_hex: &str) -> Result<String, SymmetricCipherError> {
    let iv = [7u8; 16];
    let key = [7u8; 32];
    let bytes = (0..data_hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&data_hex[i..i + 2], 16).unwrap())
        .collect::<Vec<u8>>();
    let decrypted_data = aes256_cbc_decrypt(&bytes, &key, &iv)?;

    let result = std::str::from_utf8(decrypted_data.as_slice()).unwrap();

    Ok(result.to_owned())
}

fn aes256_cbc_encrypt(
    data: &[u8],
    key: &[u8; 32],
    iv: &[u8; 16],
) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut encryptor = aes::cbc_encryptor(KeySize256, key, iv, PkcsPadding);

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}

fn aes256_cbc_decrypt(
    data: &[u8],
    key: &[u8; 32],
    iv: &[u8; 16],
) -> Result<Vec<u8>, SymmetricCipherError> {
    let mut decryptor = aes::cbc_decryptor(KeySize256, key, iv, PkcsPadding);

    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);
    let mut read_buffer = RefReadBuffer::new(data);
    let mut final_result = Vec::new();

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(
            write_buffer
                .take_read_buffer()
                .take_remaining()
                .iter()
                .map(|&i| i),
        );
        match result {
            crypto::buffer::BufferResult::BufferUnderflow => break,
            _ => continue,
        }
    }

    Ok(final_result)
}
