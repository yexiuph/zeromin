use std::fs::File;
use std::io::{self, Read, Write};
use tokio::fs::File as TokioFile;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub trait TMethod {
    // Compute Signature
    fn signature(&self, sig: &mut [char]);

    // Encryption for a string of chars
    fn encrypt(&self, input: &str, result: &mut [char]);

    // Decryption for a string of chars
    fn decrypt(&self, input: &str, result: &mut [char]);

    // Encryption for a File
    async fn encrypt_file(&self, file_in: &str, file_out: &str) -> io::Result<()>;

    // Decryption for a File
    async fn decrypt_file(&self, file_in: &str, file_out: &str) -> io::Result<()>;
     
     // Setting the Operation Mode
     fn set_mode(&mut self, mode: Mode);
     
     // Setting the Padding Mode
     fn set_padding(&mut self, padding: Padding);
     
     // Getters
     fn get_key_length(&self) -> i32;
     fn get_block_size(&self) -> i32;
     fn get_mode(&self) -> Mode;
     fn get_padding(&self) -> Padding;
     fn get_encrypt_string_length(&self, str_in: &str) -> i32;
     fn get_encrypt_length(&self, n: i32) -> i32;
     
     // Padding the input string before encryption
     fn pad(&self, input: &mut [char], length: i32) -> i32;
     
     // Resetting the Initialization Vector
     fn reset_chain(&mut self);

     fn xor(&self, buff: &mut [char], chain: &[char]);
     fn help_throw(&self, file_in: &str);
     fn bytes_to_word(&self, bytes: &[u8], word: &mut u32) -> u32;
     fn word_to_bytes(&self, word: u32, bytes: &mut [u8]);
}


#[derive(Debug,Clone, Copy)]
pub enum Mode {
    ECB,
    CBC,
    CFB,
}

#[derive(Debug, Clone, Copy)]
pub enum Padding {
    ZEROES,
    BLANKS,
    PKCS7,
}

pub struct IMethod {
    m_init : bool,
    m_block_size : i32,
    m_mode : Mode,
    m_padding: Padding
}

impl TMethod for IMethod {
    fn signature(&self, sig: &mut [char]) {
        // Implementation
    }

    fn encrypt(&self, input: &str, result: &mut [char]) {
        // Implementation
    }

    fn decrypt(&self, input: &str, result: &mut [char]) {
        // Implementation
    }

    async fn encrypt_file(&self, file_in: &str, file_out: &str) -> io::Result<()> {
        let mut input_file = TokioFile::open(file_in).await?;
        let mut output_file = TokioFile::create(file_out).await?;

        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer).await?;

        // Perform encryption here

        output_file.write_all(&buffer).await?;
        Ok(())
    }

    async fn decrypt_file(&self, file_in: &str, file_out: &str) -> io::Result<()> {
        let mut input_file = TokioFile::open(file_in).await?;

        let mut output_file = TokioFile::create(file_out).await?;

        let mut buffer = Vec::new();
        input_file.read_to_end(&mut buffer).await?;

        // Perform decryption here

        output_file.write_all(&buffer).await?;
        Ok(())
    }

    fn set_mode(&mut self, mode: Mode) {
        self.m_mode = mode;
    }

    fn set_padding(&mut self, padding: Padding) {
        self.m_padding = padding;
    }

    fn get_key_length(&self) -> i32 {
        // Implementation
        0
    }

    fn get_block_size(&self) -> i32 {
        // Implementation
        0
    }

    fn get_mode(&self) -> Mode {
        self.m_mode
    }

    fn get_padding(&self) -> Padding {
        self.m_padding
    }

    fn get_encrypt_string_length(&self, str_in: &str) -> i32 {
        // Implementation
        0
    }

    fn get_encrypt_length(&self, n: i32) -> i32 {
        // Implementation
        0
    }

    fn pad(&self, input: &mut [char], length: i32) -> i32 {
        // Implementation
        0
    }

    fn reset_chain(&mut self) {
        // Implementation
    }

    fn xor(&self, buff: &mut [char], chain: &[char]) {
        // Implementation
    }

    fn help_throw(&self, file_in: &str) {
        // Implementation
    }

    fn bytes_to_word(&self, bytes: &[u8], word: &mut u32) -> u32 {
        *word |= (bytes[0] as u32) << 24;
        *word |= (bytes[1] as u32) << 16;
        *word |= (bytes[2] as u32) << 8;
        *word |= bytes[3] as u32;
        *word
    }

    fn word_to_bytes(&self, word: u32, bytes: &mut [u8]) {
        bytes[0] = ((word >> 24) & 0xFF) as u8;
        bytes[1] = ((word >> 16) & 0xFF) as u8;
        bytes[2] = ((word >> 8) & 0xFF) as u8;
        bytes[3] = (word & 0xFF) as u8;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_to_word() {
        let method = IMethod {
            m_init: true, 
            m_block_size: 16, 
            m_mode: Mode::ECB, 
            m_padding: Padding::PKCS7, 
        };
        let mut word: u32 = 0;
        let bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
        method.bytes_to_word(&bytes, &mut word);
        assert_eq!(word, 0x12345678);
    }

    #[test]
    fn test_word_to_bytes() {
        let method = IMethod {
            m_init: true,
            m_block_size: 16,
            m_mode: Mode::ECB,
            m_padding: Padding::PKCS7,
        };
        let word: u32 = 0x51212321;
        println!("{}", word);
        let mut bytes = [0u8; 4];
        method.word_to_bytes(word, &mut bytes);
        assert_eq!(bytes, [0x51,0x21,0x23,0x21]);
    }
}