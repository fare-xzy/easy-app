use openssl::x509::X509;

#[cfg(test)]
mod tests {
    use std::fs;
    pub static RSA_PATH: &'static str = "../test-file/Trust-Sign-Sha256 CA-2 000500.cer";
    #[test]
    fn test_parse_rsa_cert(){
        let result = fs::read(&RSA_PATH)?;
        let cow = String::from_utf8(result)?;
        println!("{}",cow)
    }
}

// rsa证书解析
fn parse_rsa_cert(base64_rsa_cert: &str) {
    let mut buf = Vec::new();

    let cert = X509::from_pem(&buf).expect("Unable to parse certificate");

    println!("Certificate: {:?}", cert);
}