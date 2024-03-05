use crc32fast::Hasher;
use gm_sm2::error::Sm2Result;
use gm_sm2::key::{Sm2Model, Sm2PrivateKey};
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

#[cfg(test)]
mod tests {
    use std::fs;
    use base64::DecodeError;
    use crate::bjca_decrypt::{crc32_verify, decrypt_rsa_keys, decrypt_sm2_keys, json_deserialize};
    use crate::conts;
    use base64::prelude::*;
    use gm_sm2::key::{gen_keypair, Sm2Model};

    pub static RSA_PATH: &'static str = "../手写签名加密包RSA.txt";
    pub static SM2_PATH: &'static str = "../手写签名加密包SM2.txt";
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
                let rsa_key = conts::RSA_KEY.clone();
                let result1 = BASE64_STANDARD.decode(encrypt_package.EncKey);
                match result1 {
                    Ok(enc) => {
                        decrypt_rsa_keys(conts::RSA_KEY.private_key, enc);
                    }
                    Err(_) => {}
                }

            }
            Err(_) => {}
        }
        Ok(())
    }
    #[test]
    fn test_decrypt_sm2() -> Result<(), Box<dyn std::error::Error>>{
        let result = fs::read(&SM2_PATH)?;
        let cow = String::from_utf8(result)?;
        let dese_resutl = json_deserialize(&cow);
        match dese_resutl {
            Ok(encrypt_package) => {
                let rsa_key = conts::RSA_KEY.clone();
                let enc = BASE64_STANDARD.decode(encrypt_package.EncKey)?;
                let pk = BASE64_STANDARD.decode(conts::SM2_KEY.private_key)?;
                decrypt_sm2_keys(&hex::encode(pk), enc).expect("TODO: panic message");
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
fn decrypt_rsa_keys(private_str: &str, encrypt_data: Vec<u8>) -> Vec<u8>{
    let private_key = RsaPrivateKey::from_pkcs8_pem(&*private_str);
    let dec_data = private_key.unwrap().decrypt(Pkcs1v15Encrypt, &encrypt_data).expect("failed to decrypt");
    dec_data
}

// SM2解密
fn decrypt_sm2_keys(private_str_hex: &str, encrypt_data: Vec<u8>) -> Sm2Result<Vec<u8>> {
    let private_key = Sm2PrivateKey::from_hex_string(private_str_hex).unwrap();
    let dec_data = private_key.decrypt(&encrypt_data, false, Sm2Model::C1C3C2);
    dec_data
}