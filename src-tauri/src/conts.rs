// 定义 SecretKey 结构体
pub struct SecretKey {
    pub sn: &'static str,
    pub private_key: &'static str,
}

// 实现 Clone trait
impl Clone for SecretKey {
    fn clone(&self) -> Self {
        SecretKey {
            sn: self.sn.clone(),
            private_key: self.private_key.clone(),
        }
    }
}

// 定义全局变量 RSA_KEY 和 SM2KEY
pub static RSA_KEY: SecretKey = SecretKey {
    sn: "1B40000000000005DB00",
    private_key: "
-----BEGIN PRIVATE KEY-----
MIICdgIBADANBgkqhkiG9w0BAQEFAASCAmAwggJcAgEAAoGBAIqkQ52yM2eow5Au
WBwGR2Vup75JNoOgNIIX+QsEcUVXnO4TY9QvWtJo4y7ZxlvuofOY7v96tLYx9Ksu
sNRDgCKfFf6ZC0e8jHT83vOf3XJr3C+g/cREn+tqZbWgz9UNwxCGDqluNdsGN22o
IgyGlKHUo2r7EKbh82cwfMwMmbvRAgMBAAECgYAlN61zEthb1mERPm+mrkTSRN9A
5LR+py4RUlAnlfy2Sau1+XyOBFxUKZX7CLkICsps8zwNVypV0plErtcHVEt8Gh2b
d6wu3JaOHXdUw2lKp6uUWkAaRMSLAoE6xT+eGMlwMTfz6GvqDvOT5kKaQB66+fRP
W3FcjGg4isqNdkIsRQJBANCoUdoQGuTkqxmFm9oqU2kXCPZGLwaZcxCJCv/eG54W
8FEN2tGkjx5lHNtWz+gKuYbAngGe7ubL5d04hfgWrEcCQQCqGSNdGZvMiD7wHlWQ
cPwaWrGegIN2agD8ipfwHKU/bV3Z4ZQJktFBTWvaZFoiHH7IgU2GDpTI9LUgpDxK
NhsnAkEAlbS+jXNSqHl8XWJkUOJfWB4j2va+FMESMoHw92W3cPTKVY1YO2QTH16y
lpixsK3JMOqQM1+BzuvqL4+3Zyl1TwJAZdEoTukuTi7b5aVyuEQ5U5koKuVoS7BG
KOTY79xlueLAQ9R4ZYahhUJPpeld3n9KEMD9Y5pEy0lXRlOo1w8uDwJAFaAPJ71R
UiYN7zrv/a1OvqdyYNCQB51bkEWqSy9qQ6+mg7w1xk52OTR2fZMXqreMf2IPDcr0
roWUvZt6hBs42g==
-----END PRIVATE KEY-----
",
};

pub static SM2_KEY: SecretKey = SecretKey {
    sn: "1A100000000001C80EF2",
    private_key: "j/UMBbgd3BQ7Y9BX4iALBZlj1xfsG+aWiSEryN80NWU=",
};