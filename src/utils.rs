use std::io;
use hudsucker::{
    certificate_authority::OpensslAuthority,
    openssl::{hash::MessageDigest, pkey::PKey, x509::X509, self},
    self
};
use std::fmt;
use std::fs;
use std::io::Read;

#[derive(Debug)]
pub(crate) enum CrusterError {
    IOError(String),
    OpenSSLError(String),
    HudSuckerError(String),
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

impl fmt::Display for CrusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
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
