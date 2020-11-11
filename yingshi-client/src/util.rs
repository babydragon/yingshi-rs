use super::Result;
use md5::{Md5, Digest};
use aes::Aes128;
use block_modes::{Cbc, block_padding::Pkcs7, BlockMode};
use std::fmt::Write;
use crate::APIError;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// 解码数据
pub fn decrypt<'a>(data: &'a mut [u8], verify_code: &str) -> Result<&'a [u8]> {
    if data.len() <= 48 { 
        return Err(APIError{
            code: "901".to_string(),
            message: "invalid crypt data".to_string(),
        });
    }
    // 验证加密文件是否应该使用当前verify code
    let first_md5 = md5_string(verify_code);
    let verify_code_md5 = md5_string(first_md5.as_str());
    let md5_in_file = std::str::from_utf8(&data[16..48]).unwrap();

    if verify_code_md5 != md5_in_file {
        return Err(APIError{
            code: "902".to_string(),
            message: "crypt data and verify code not match".to_string(),
        });
    }

    // 开始解密
    let iv: [u8; 16] = [48, 49, 50, 51, 52, 53, 54, 55, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut key = [0u8; 16];
    let code_slice = verify_code.as_bytes();
    let index = std::cmp::min(code_slice.len(), 16);
    key[..index].copy_from_slice(code_slice);

    let content = &mut data[48..];
    let cipher = Aes128Cbc::new_var(&key, &iv).unwrap();
    let result = cipher.decrypt(content);

    match result {
        Ok(text) => {
            Ok(text)
        }
        Err(_) => {
            Err(APIError{
                code: "903".to_string(),
                message: "decrypt fail".to_string(),
            })
        }
    }
}

fn md5_string(s: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(s.as_bytes());
    let digest = hasher.finalize();

    let mut result = String::with_capacity(2 * digest.len());
    for byte in &digest {
        write!(result, "{:02x}", byte).unwrap();
    }

    result
}