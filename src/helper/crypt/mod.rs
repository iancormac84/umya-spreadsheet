//! Cryptographic utilities for Excel workbook and worksheet protection.
//!
//! This module provides functionality for encrypting and managing protection
//! settings in Excel workbooks and worksheets. It includes methods for:
//! - Password-based encryption
//! - Hash generation
//! - Salt management
//! - Protection settings configuration
//!
//! The encryption process follows Microsoft's Office Open XML standards for
//! document protection, using standardized algorithms and key derivation
//! functions.
//!
//! # Examples
//!
//! ```
//! use crate::{
//!     helper::crypt,
//!     structs::SheetProtection,
//! };
//!
//! let mut protection = SheetProtection::new();
//! crypt::encrypt_sheet_protection("mypassword", &mut protection);
//! ```
//!
//! # Security
//!
//! The module implements industry-standard cryptographic practices:
//! - Random salt generation for each encryption
//! - Configurable key spin count for key derivation
//! - Base64 encoding for binary data storage
//! - Secure password hashing

use std::{
    io::Write,
    path::Path,
};

use rand::Rng;

use crate::structs::{
    SheetProtection,
    WorkbookProtection,
};

pub(crate) mod algo;
pub(crate) mod constants;
pub(crate) mod key;
pub(crate) mod utils;

use base64::{
    Engine as _,
    engine::general_purpose::STANDARD,
};
use utils::generate_random_bytes;

/// Encrypts the sheet protection using the provided password.
///
/// Updates the `sheet_protection` object with the algorithm name, salt value,
/// spin count, and hash value.
///
/// # Parameters
///
/// - `password`: The password to use for encryption.
/// - `sheet_protection`: The `SheetProtection` object to update with encryption
///   details.
#[allow(clippy::cast_possible_truncation)]
pub fn encrypt_sheet_protection(password: &str, sheet_protection: &mut SheetProtection) {
    generate_random_bytes!(salt, 16);

    // Convert the password into a hash
    let key = key::convert_password_to_hash(password, &salt, constants::KEY_SPIN_COUNT);

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(salt);
    let hash_value_str = STANDARD.encode(&key);

    // Update the sheet_protection object
    sheet_protection.set_algorithm_name(constants::KEY_HASH_ALGORITHM);
    sheet_protection.set_salt_value(salt_value_str);
    sheet_protection.set_spin_count(constants::KEY_SPIN_COUNT as u32);
    sheet_protection.set_hash_value(hash_value_str);
    sheet_protection.remove_password_raw();
}

/// Encrypts the workbook protection using the provided password.
///
/// Updates the `workbook_protection` object with the algorithm name, salt
/// value, spin count, and hash value.
///
/// # Parameters
///
/// - `password`: The password to use for encryption.
/// - `workbook_protection`: The `WorkbookProtection` object to update with
///   encryption details.
#[allow(clippy::cast_possible_truncation)]
pub fn encrypt_workbook_protection(password: &str, workbook_protection: &mut WorkbookProtection) {
    generate_random_bytes!(salt, 16);

    // Convert the password into a hash
    let key = key::convert_password_to_hash(password, &salt, constants::KEY_SPIN_COUNT);

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(salt);
    let hash_value_str = STANDARD.encode(&key);

    // Update the workbook_protection object
    workbook_protection.set_workbook_algorithm_name(constants::KEY_HASH_ALGORITHM);
    workbook_protection.set_workbook_salt_value(salt_value_str);
    workbook_protection.set_workbook_spin_count(constants::KEY_SPIN_COUNT as u32);
    workbook_protection.set_workbook_hash_value(hash_value_str);
    workbook_protection.remove_workbook_password_raw();
}

/// Encrypts the revisions protection using the provided password.
///
/// This function generates a random salt, derives a key from the password using
/// SHA-512, and updates the workbook protection settings with the encrypted
/// values.
///
/// # Arguments
///
/// * `password` - The password string to use for encryption
/// * `workbook_protection` - Mutable reference to the `WorkbookProtection`
///   object to update
#[allow(clippy::cast_possible_truncation)]
pub fn encrypt_revisions_protection(password: &str, workbook_protection: &mut WorkbookProtection) {
    generate_random_bytes!(salt, 16);

    // Convert the password into a hash
    let key = key::convert_password_to_hash(password, &salt, constants::KEY_SPIN_COUNT);

    // Encode the salt and hash value in base64
    let salt_value_str = STANDARD.encode(salt);
    let hash_value_str = STANDARD.encode(&key);

    // Update the workbook_protection object
    workbook_protection.set_revisions_algorithm_name(constants::KEY_HASH_ALGORITHM);
    workbook_protection.set_revisions_salt_value(salt_value_str);
    workbook_protection.set_revisions_spin_count(constants::KEY_SPIN_COUNT as u32);
    workbook_protection.set_revisions_hash_value(hash_value_str);
    workbook_protection.remove_revisions_password_raw();
}

/// Encrypts the given data and writes it to a specified file.
///
/// This function performs the following steps:
/// 1. Generates random bytes for various cryptographic keys and salts.
/// 2. Encrypts the data using a specified package cipher algorithm (AES).
/// 3. Generates and encrypts an HMAC key and its corresponding value for data
///    integrity.
/// 4. Converts the provided password into a key using a key derivation
///    function.
/// 5. Encrypts the derived key and generates a verifier hash input and value,
///    both of which are also encrypted.
/// 6. Constructs an XML structure containing encryption information.
/// 7. Creates a compound file and writes the encrypted data and metadata to it.
///
/// # Parameters
///
/// - `filepath`: A reference to the path where the encrypted data will be
///   saved. This can be any type that implements the `AsRef<Path>` trait.
/// - `data`: A byte slice containing the data to be encrypted.
/// - `password`: A string slice representing the password used for key
///   derivation.
///
/// # Errors
///
/// This function may return errors related to file I/O or cryptographic
/// operations. Ensure to handle these errors appropriately in your application.
///
/// # Panics
///
/// This function may panic if:
/// - The underlying file operations fail (e.g., if the file cannot be created
///   or written to).
/// - Any cryptographic operation fails, such as key generation or encryption,
///   due to invalid parameters or internal errors.
///
/// # Example
///
/// ```
/// let data = b"Sensitive data to encrypt";
/// let password = "securepassword";
/// let filepath = "encrypted_data.bin";
///
/// encrypt(&filepath, data, password);
/// ```
///
/// # Note
///
/// The encryption process involves multiple cryptographic operations, including
/// key generation, HMAC creation, and data encryption. Ensure that the password
/// used is strong and kept secure.
pub fn encrypt<P: AsRef<Path>>(filepath: &P, data: &[u8], password: &str) {
    generate_random_bytes!(hmac_key, 64);
    generate_random_bytes!(key_salt, 16);
    generate_random_bytes!(package_key, 32);
    generate_random_bytes!(package_salt, 16);
    generate_random_bytes!(verifier_hash_input, 16);

    // Encrypt the package
    let encrypted_package = algo::crypt_package(
        true,
        constants::PACKAGE_BLOCK_SIZE,
        &package_salt,
        &package_key,
        data,
    );

    // Generate HMAC key and encrypt it
    let hmac_key_iv = key::create_iv(
        &package_salt,
        constants::PACKAGE_BLOCK_SIZE,
        &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
    );
    let encrypted_hmac_key = algo::crypt(true, &package_key, &hmac_key_iv, &hmac_key).unwrap();

    // Generate HMAC value and encrypt it
    let hmac_value = key::hmac(&hmac_key, &[&encrypted_package]);
    let hmac_value_iv = key::create_iv(
        &package_salt,
        constants::PACKAGE_BLOCK_SIZE,
        &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE,
    );
    let encrypted_hmac_value =
        algo::crypt(true, &package_key, &hmac_value_iv, &hmac_value).unwrap();

    // Convert the password to a key
    let key = key::convert_password_to_key(
        password,
        &key_salt,
        constants::KEY_SPIN_COUNT,
        constants::KEY_BITLENGTH,
        &constants::BLOCK_KEYS_KEY,
    );
    let encrypted_key_value = algo::crypt(true, &key, &key_salt, &package_key).unwrap();

    // Generate verifier hash input and encrypt it
    let verifier_hash_input_key = key::convert_password_to_key(
        password,
        &key_salt,
        constants::KEY_SPIN_COUNT,
        constants::KEY_BITLENGTH,
        &constants::BLOCK_VERIFIER_HASH_INPUT,
    );
    let encrypted_verifier_hash_input = algo::crypt(
        true,
        &verifier_hash_input_key,
        &key_salt,
        &verifier_hash_input,
    )
    .unwrap();

    // Generate verifier hash value and encrypt it
    let verifier_hash_value = utils::hash_concatenated(&[&verifier_hash_input]);
    let verifier_hash_value_key = key::convert_password_to_key(
        password,
        &key_salt,
        constants::KEY_SPIN_COUNT,
        constants::KEY_BITLENGTH,
        &constants::BLOCK_VERIFIER_HASH_VALUE,
    );
    let encrypted_verifier_hash_value = algo::crypt(
        true,
        &verifier_hash_value_key,
        &key_salt,
        &verifier_hash_value,
    )
    .unwrap();

    // Build the encryption info XML data
    let encryption_info_buffer = algo::build_encryption_info(
        &package_salt,
        &encrypted_hmac_key,
        &encrypted_hmac_value,
        &key_salt,
        &encrypted_verifier_hash_input,
        &encrypted_verifier_hash_value,
        &encrypted_key_value,
    );

    // Create compound file and write streams
    let mut comp = cfb::create(filepath).unwrap();
    {
        let mut stream_info = comp.create_stream("EncryptionInfo").unwrap();
        stream_info.write_all(&encryption_info_buffer).unwrap();
    }
    {
        let mut stream_package = comp.create_stream("EncryptedPackage").unwrap();
        stream_package.write_all(&encrypted_package).unwrap();
    }
}

#[allow(unused_macros)]
#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::Read,
    };

    use hex_literal::hex;

    /// Prints a byte slice as a hex string, prefixed with the variable name.
    ///
    /// This macro takes a reference to a byte slice (`&[u8]`) and prints both
    /// the variable name and its hexadecimal representation to stdout.
    macro_rules! print_hex {
        ($var:expr) => {
            println!(
                "{} = {}",
                stringify!($var),
                $var.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            );
        };
    }

    use super::*;

    #[test]
    fn test_encrypt() {
        let mut file = File::open("./tests/test_files/aaa.xlsx").unwrap();
        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();

        let password = "password";

        // Package parameters
        let package_key = hex!("cdf9defae2480933c503350e16334453d1cb8348bb2fea585db7f9e1f78fe9bf");
        let package_salt = hex!("4c251b321d85cecfcb6d952ba6d81846");

        // Key parameters
        let key_salt = hex!("3aa973eec73c98c4710021730ef5b513");

        // Encrypted package
        let encrypted_package = algo::crypt_package(
            true,
            constants::PACKAGE_BLOCK_SIZE,
            &package_salt,
            &package_key,
            &data,
        );

        // HMAC key
        let hmac_key = hex!(
            "4c6e4db6d9a60e5d41c3ca639a682aaa71da7437202fe92ec5d814bd1e9e4e6a"
            "831aee889eae3bc18bc1bebedae1f73393fddfffd0a0b6c557485fefcdb5e98b"
        );

        let hmac_key_iv = key::create_iv(
            &package_salt,
            constants::PACKAGE_BLOCK_SIZE,
            &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
        );
        assert_eq!(hmac_key_iv, hex!("ba1bf00eed82b07ee65e574eb1f46043"));

        let encrypted_hmac_key = algo::crypt(true, &package_key, &hmac_key_iv, &hmac_key).unwrap();
        assert_eq!(
            encrypted_hmac_key,
            hex!(
                "b32b1cdc4ac1af244377c1eb57efd31a819f555a7204adcc0cfe364b394bbdb0
                 86a8daef4f4c512d52e3db6a54b1d45e1dd1dbfa3ddacc29fe35449ba5225dc7"
            )
        );

        // HMAC value
        let hmac_value = key::hmac(&hmac_key, &[&encrypted_package]);
        assert_eq!(
            hmac_value,
            hex!(
                "48ed7b8718e55f7f6e19e1cae6a447afacff5e5e3fe92ac836908e6cdeeb68a2"
                "5fa1cbaaa4d7f7c0acabc2f7f22ad87bf11eaf74169fb6e6f78ab033e038928c"
            )
        );

        let hmac_value_iv = key::create_iv(
            &package_salt,
            constants::PACKAGE_BLOCK_SIZE,
            &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_VALUE,
        );
        assert_eq!(hmac_value_iv, hex!("088385b871292e7ed8414f173c5b6622"));

        let encrypted_hmac_value =
            algo::crypt(true, &package_key, &hmac_value_iv, &hmac_value).unwrap();
        assert_eq!(
            encrypted_hmac_value,
            hex!(
                "f75c7f3c44fadf9b4bbf2ff693586710c52e043d8db69e3e538be5f10d36f86d"
                "24dd5f4b9f71a2ce928abbbfe46e791a6c683703bcb30d5214997e60bbd547f6"
            )
        );

        // Key
        let key = key::convert_password_to_key(
            password,
            &key_salt,
            constants::KEY_SPIN_COUNT,
            constants::KEY_BITLENGTH,
            &constants::BLOCK_KEYS_KEY,
        );
        assert_eq!(
            key,
            hex!("8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d")
        );

        let encrypted_key_value = algo::crypt(true, &key, &key_salt, &package_key).unwrap();
        assert_eq!(
            encrypted_key_value,
            hex!("5017ddc6146e56dfbf76734b3e99b80f36a4c9a2e9eb21fe77695f73850cc452")
        );

        // Verifier hash input
        let verifier_hash_input = hex!("8f54777cba87efa55ea2db8399873815");
        let verifier_hash_input_key = key::convert_password_to_key(
            password,
            &key_salt,
            constants::KEY_SPIN_COUNT,
            constants::KEY_BITLENGTH,
            &constants::BLOCK_VERIFIER_HASH_INPUT,
        );
        assert_eq!(
            verifier_hash_input_key,
            hex!("44e4b664c512b08e7577aa3fc7e11ad603e0877a476931fad5aa79e203304aff")
        );

        let encrypted_verifier_hash_input = algo::crypt(
            true,
            &verifier_hash_input_key,
            &key_salt,
            &verifier_hash_input,
        )
        .unwrap();
        assert_eq!(
            encrypted_verifier_hash_input,
            hex!("2fb9eea58e227ffa549449e941f1199e")
        );

        // Verifier hash value
        let verifier_hash_value = utils::hash_concatenated(&[&verifier_hash_input]);
        assert_eq!(
            verifier_hash_value,
            hex!(
                "920b1de74f38d9cb3ccb3394119ed37e958404fdc47560b1bf647d3c49c22549"
                "625fe4a0bd36798bd68a0d98ae64f6ab64a330c9890c62bb740aa492c226ae1f"
            )
        );

        let verifier_hash_value_key = key::convert_password_to_key(
            password,
            &key_salt,
            constants::KEY_SPIN_COUNT,
            constants::KEY_BITLENGTH,
            &constants::BLOCK_VERIFIER_HASH_VALUE,
        );
        assert_eq!(
            verifier_hash_value_key,
            hex!("d5515a6062e3e99551b80b92db1fe646483884cdb63e1e7595a9f2cca7532884")
        );

        let encrypted_verifier_hash_value = algo::crypt(
            true,
            &verifier_hash_value_key,
            &key_salt,
            &verifier_hash_value,
        )
        .unwrap();
        assert_eq!(
            encrypted_verifier_hash_value,
            hex!(
                "0d9c888111b40b630b739c95a5f5b6be67c8f96acdd1bee185bd808b507f6527"
                "60a2e77f63a6ad0c46f985f2bb8dab4fcf9b86d6a40d9c21299bb4ddf788b250"
            )
        );

        // Build encryption info
        let encryption_info = algo::build_encryption_info(
            &package_salt,
            &encrypted_hmac_key,
            &encrypted_hmac_value,
            &key_salt,
            &encrypted_verifier_hash_input,
            &encrypted_verifier_hash_value,
            &encrypted_key_value,
        );
        assert_eq!(
            encryption_info,
            hex!(
                "04000400400000003c3f786d6c2076657273696f6e3d22312e302220656e636f"
                "64696e673d225554462d3822207374616e64616c6f6e653d22796573223f3e0d"
                "0a3c656e6372797074696f6e20786d6c6e733d22687474703a2f2f736368656d"
                "61732e6d6963726f736f66742e636f6d2f6f66666963652f323030362f656e63"
                "72797074696f6e2220786d6c6e733a703d22687474703a2f2f736368656d6173"
                "2e6d6963726f736f66742e636f6d2f6f66666963652f323030362f6b6579456e"
                "63727970746f722f70617373776f72642220786d6c6e733a633d22687474703a"
                "2f2f736368656d61732e6d6963726f736f66742e636f6d2f6f66666963652f32"
                "3030362f6b6579456e63727970746f722f6365727469666963617465223e3c6b"
                "6579446174612073616c7453697a653d2231362220626c6f636b53697a653d22"
                "313622206b6579426974733d2232353622206861736853697a653d2236342220"
                "636970686572416c676f726974686d3d22414553222063697068657243686169"
                "6e696e673d22436861696e696e674d6f6465434243222068617368416c676f72"
                "6974686d3d22534841353132222073616c7456616c75653d22544355624d6832"
                "467a732f4c625a55727074675952673d3d222f3e3c64617461496e7465677269"
                "747920656e63727970746564486d61634b65793d227379736333457242727952"
                "4464384872562b2f54476f476656567079424b334d44503432537a6c4c766243"
                "47714e7276543078524c564c6a3232705573645265486448622b6a33617a436e"
                "2b4e55536270534a6478773d3d2220656e63727970746564486d616356616c75"
                "653d223931782f504554363335744c76792f326b31686e454d55754244324e74"
                "70342b5534766c385130322b47306b3356394c6e3347697a704b4b75372f6b62"
                "6e6b61624767334137797a445649556d5835677539564839673d3d222f3e3c6b"
                "6579456e63727970746f72733e3c6b6579456e63727970746f72207572693d22"
                "687474703a2f2f736368656d61732e6d6963726f736f66742e636f6d2f6f6666"
                "6963652f323030362f6b6579456e63727970746f722f70617373776f7264223e"
                "3c703a656e637279707465644b6579207370696e436f756e743d223130303030"
                "30222073616c7453697a653d2231362220626c6f636b53697a653d2231362220"
                "6b6579426974733d2232353622206861736853697a653d223634222063697068"
                "6572416c676f726974686d3d224145532220636970686572436861696e696e67"
                "3d22436861696e696e674d6f6465434243222068617368416c676f726974686d"
                "3d225348412d353132222073616c7456616c75653d224f716c7a377363386d4d"
                "52784143467a4476573145773d3d2220656e6372797074656456657269666965"
                "7248617368496e7075743d224c376e7570593469662f70556c456e705166455a"
                "6e673d3d2220656e6372797074656456657269666965724861736856616c7565"
                "3d22445a79496752473043324d4c6335795670665732766d66492b57724e3062"
                "3768686232416931422f5a5364676f75642f593661744445623568664b376a61"
                "74507a3575473171514e6e4345706d3754643934697955413d3d2220656e6372"
                "79707465644b657956616c75653d22554266647868527556742b2f646e4e4c50"
                "706d34447a616b79614c703679482b64326c666334554d7846493d222f3e3c2f"
                "6b6579456e63727970746f723e3c2f6b6579456e63727970746f72733e3c2f65"
                "6e6372797074696f6e3e"
            )
        );
    }

    #[test]
    fn test_hash() {
        let package_salt = hex!("4c251b321d85cecfcb6d952ba6d81846");
        let result = utils::hash_concatenated(&[
            &package_salt,
            &constants::BLOCK_KEYS_DATA_INTEGRITY_HMAC_KEY,
        ]);
        assert_eq!(
            result,
            hex!(
                "ba1bf00eed82b07ee65e574eb1f460435d2a1405e81904fd01d5ed5adf43fdcf
                 d8e9aeebad0c08065e0db20cdc8e4552744b61ad1b3cf9a3c5aad5b2a047e76b"
            )
        );
    }

    #[test]
    fn test_convert_password_to_key() {
        let key_salt = hex!("3aa973eec73c98c4710021730ef5b513");
        let result = key::convert_password_to_key(
            "password",
            &key_salt,
            100_000,
            256,
            &constants::BLOCK_KEYS_KEY,
        );
        assert_eq!(
            result,
            hex!("8d5869311b1c1fdb59a1de6fe1e6f2ce7dccd4deb198a6dfb1f7fb55bc03487d")
        );
    }
}
