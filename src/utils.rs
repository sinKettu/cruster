use hudsucker::{
    certificate_authority::OpensslAuthority,
    openssl::{hash::MessageDigest, pkey::PKey, x509::X509, self},
    self
};
use std::{
    io::{self, Read},
    fmt::{self, Debug},
    num::ParseIntError,
    net::AddrParseError,
    fs
};
use rcgen;
use serde_yaml;

#[derive(Debug)]
pub(crate) enum CrusterError {
    IOError(String),
    OpenSSLError(String),
    HudSuckerError(String),
    ConfigError(String),
    PortParsingError(String),
    ParseAddressError(String),
    UndefinedError(String)
}

impl From<io::Error> for CrusterError {
    fn from(e: io::Error) -> Self { Self::IOError(e.to_string()) }
}

impl From<openssl::error::Error> for CrusterError {
    fn from(e: openssl::error::Error) -> Self { Self::OpenSSLError(e.to_string()) }
}

impl From<openssl::error::ErrorStack> for CrusterError {
    fn from(e: openssl::error::ErrorStack) -> Self { Self::OpenSSLError(e.to_string()) }
}

impl From<hudsucker::Error> for CrusterError {
    fn from(e: hudsucker::Error) -> Self { Self::HudSuckerError(e.to_string()) }
}

impl From<String> for CrusterError {
    fn from(s: String) -> Self { Self::UndefinedError(s.to_string()) }
}

impl From<ParseIntError> for CrusterError {
    fn from(e: ParseIntError) -> Self {
        Self::PortParsingError(
            format!("Unable to parse port number: {}", e)
        )
    }
}

impl From<AddrParseError> for CrusterError {
    fn from(e: AddrParseError) -> Self { Self::ParseAddressError(e.to_string()) }
}

impl From<serde_yaml::Error> for CrusterError {
    fn from(e: serde_yaml::Error) -> Self {
        Self::ConfigError(
            format!("Unable to serialize/deserialize YAML data: {}", e.to_string())
        )
    }
}

impl fmt::Display for CrusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CrusterError::ConfigError(s) => {
                write!(f, "An error occurred while handling input parameters: {}\n{}",
                       s,
                       "Enter '-h' for help."
                )
            },
            _ => { write!(f, "{}", self) }
        }
    }
}

// ---------------------------------------------------------------------------------------------- //

pub(crate) fn get_ca(key_path: &str, cer_path: &str) -> Result<OpensslAuthority, CrusterError> {
    let mut key_buffer: Vec<u8> = Vec::new();
    let f = fs::File::open(key_path);
    match f {
        Ok(mut file) => {
            let res = file.read_to_end(&mut key_buffer);
            if let Err(e) = res {
                return Err(
                    CrusterError::IOError(
                        format!("Could not read from key file, info: {}", e.to_string())
                    )
                )
            }
        },
        Err(e) => return Err(
            CrusterError::IOError(
                format!("Could not find or open key file, info: {}", e.to_string())
            )
        )
    }
    let private_key_bytes: &[u8] = &key_buffer;
    let mut cer_buffer: Vec<u8> = Vec::new();
    let f = fs::File::open(cer_path);
    match f {
        Ok(mut file) => {
            let res = file.read_to_end(&mut cer_buffer);
            if let Err(e) = res {
                return Err(
                    CrusterError::IOError(
                        format!("Could not read from cer file, info: {}", e.to_string())
                    )
                )
            }
        },
        Err(e) => return Err(
            CrusterError::IOError(
                format!("Could not find or open cer file, info: {}", e.to_string())
            )
        )
    }
    let ca_cert_bytes: &[u8] = &cer_buffer;

    let private_key = PKey::private_key_from_pem(private_key_bytes)?;
    let ca_cert = X509::from_pem(ca_cert_bytes)?;

    Ok(OpensslAuthority::new(
        private_key,
        ca_cert,
        MessageDigest::sha256(),
        1_000
    ))
}

pub(crate) fn generate_key_and_cer(key_path: &str, cer_path: &str) {
    let cert: rcgen::Certificate = rcgen::generate_simple_self_signed(
        vec![
            String::from("cruster.intercepting.proxy"),
            String::from("localhost"),
            String::from("127.0.0.1")
        ]
    ).expect("Could not generate certificate, check filenames");

    // TODO: check existence!
    fs::write(
        cer_path,
        cert
            .serialize_pem()
            .expect("Unable to serialize cer-data to PEM")
            .as_bytes()
    ).expect(format!("Could not write cer-file to '{}'", cer_path.to_string()).as_str());

    // TODO: check existence!
    fs::write(
        key_path,
            cert
                .serialize_private_key_pem()
                .as_bytes()
    ).expect(format!("Could not write key-file to '{}'", key_path.to_string()).as_str());
}
