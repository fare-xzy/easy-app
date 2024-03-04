use crc32fast::Hasher;
use rand::prelude::StdRng;
use rsa::rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct EncryptPackage {
    Digest: Digest,
    EncCertSN: String,
    EncData: String,
    EncKey: String,
    TermSrc: String,
    Version: String
}
#[derive(Serialize, Deserialize)]
struct Digest {
    Alg: String,
    Value: String
}

struct SecretKey {
    sn: String,
    private_key: String
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::bjca_decrypt::{crc32_verify, json_deserialize};
    pub static RSA_PATH: &'static str = "../手写签名加密包RSA.txt";
    #[test]
    fn test_crc32() -> Result<(), Box<dyn std::error::Error>>{
        let result = fs::read(&RSA_PATH)?;
        let cow = String::from_utf8(result)?;
        let dese_resutl = json_deserialize(&cow);
        match dese_resutl {
            Ok(encrypt_package) => {
                let result1 = crc32_verify(&encrypt_package.EncData, &encrypt_package.Digest.Value);
                match result1 {
                    Ok(_) => {
                        println!("CRC32验证成功")
                    }
                    Err(err) => {
                        println!("{}", err)
                    }
                }
            }
            Err(_) => {}
        }
        Ok(())
    }

    #[test]
    fn test_json() -> Result<(), Box<dyn std::error::Error>>{
        let result = fs::read(&RSA_PATH)?;
        let cow = String::from_utf8(result)?;
        let dese_result = json_deserialize(&cow);
        match dese_result {
            Ok(encrypt_package) => {
                println!("{}", 11);
            }
            Err(_) => {}
        }
        println!("{}", cow);
        Ok(())
    }

    #[test]
    fn test_decrypt_rsa() -> Result<(), Box<dyn std::error::Error>>{
        let result = fs::read(&RSA_PATH)?;
        let cow = String::from_utf8(result)?;
        let dese_resutl = json_deserialize(&cow);
        match dese_resutl {
            Ok(encrypt_package) => {
                println!("{}", encrypt_package.EncKey)
            }
            Err(_) => {}
        }
        Ok(())
    }
}

// json反序列化
fn json_deserialize(json_str: &str) -> Result<EncryptPackage, serde_json::Error> {
    let encrypt_package: EncryptPackage = serde_json::from_str(json_str)?;
    Ok(encrypt_package)
}

// 计算CRC32
fn calculate_crc32(data: &str) -> String {
    let mut hasher = Hasher::new();
    hasher.update(data.as_bytes());
    let checksum = hasher.finalize();
    // 转化为小写的十六进制格式
    format!("{:x}", checksum)
}

// CRC32 值对比
fn crc32_verify(enc_data: &str, expected_digest: &str) -> Result<(), &'static str> {
    let format_int = calculate_crc32(enc_data);

    // CRC32 验证
    if format_int.eq_ignore_ascii_case(expected_digest) {
        Ok(())
    } else {
        Err("CRC32错误")
    }
}

extern crate rsa;
extern crate rand;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use rsa::pkcs8::DecodePrivateKey;

struct PaddingScheme(Option<StdRng>);



// RSA解密
fn decrypt_package(private_str: &str, encrypt_data: &[u8]) {
    let private_key = RsaPrivateKey::from_pkcs8_pem(private_str);
    let dec_data = private_key.unwrap().decrypt(Pkcs1v15Encrypt, &encrypt_data).expect("failed to decrypt");
}