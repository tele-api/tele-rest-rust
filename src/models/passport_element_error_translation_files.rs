//! # Telegram Bot API - REST API Client
//! 
//! Auto-generated OpenAPI schema
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-01T14:14:23.986122366Z[Etc/UTC]
//! - **Generator Version**: 7.14.0
//!
//! <details>
//! <summary><strong>⚠️ Important Disclaimer & Limitation of Liability</strong></summary>
//! <br>
//! > **IMPORTANT**: This software is provided "as is" without any warranties, express or implied, including but not limited
//! > to warranties of merchantability, fitness for a particular purpose, or non-infringement. The developers, contributors,
//! > and licensors (collectively, "Developers") make no representations regarding the accuracy, completeness, or reliability
//! > of this software or its outputs.
//! >
//! > This client is not intended to provide financial, investment, tax, or legal advice. It facilitates interaction with the
//! > Telegram Bot API service but does not endorse or recommend any financial actions, including the purchase, sale, or holding of
//! > financial instruments (e.g., stocks, bonds, derivatives, cryptocurrencies). Users must consult qualified financial or
//! > legal professionals before making decisions based on this software's outputs.
//! >
//! > Financial markets are inherently speculative and carry significant risks. Using this software in trading, analysis, or
//! > other financial activities may result in substantial losses, including total loss of capital. The Developers are not
//! > liable for any losses or damages arising from such use. Users assume full responsibility for validating the software's
//! > outputs and ensuring their suitability for intended purposes.
//! >
//! > This client may rely on third-party data or services (e.g., market feeds, APIs). The Developers do not control or verify
//! > the accuracy of these services and are not liable for any errors, delays, or losses resulting from their use. Users must
//! > comply with third-party terms and conditions.
//! >
//! > Users are solely responsible for ensuring compliance with all applicable financial, tax, and regulatory requirements in
//! > their jurisdiction. This includes obtaining necessary licenses or approvals for trading or investment activities. The
//! > Developers disclaim liability for any legal consequences arising from non-compliance.
//! >
//! > To the fullest extent permitted by law, the Developers shall not be liable for any direct, indirect, incidental,
//! > consequential, or punitive damages arising from the use or inability to use this software, including but not limited to
//! > loss of profits, data, or business opportunities.
//!
//! </details>
use crate::models;
use serde::{Deserialize, Serialize};

/// PassportElementErrorTranslationFiles : Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Error source, must be *translation\\_files*
    #[serde(rename = "source")]
    pub source: String,
    /// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// List of base64-encoded file hashes
    #[serde(rename = "file_hashes")]
    pub file_hashes: Vec<String>,
    /// Error message
    #[serde(rename = "message")]
    pub message: String,
}

impl PassportElementErrorTranslationFiles {
    /// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
    pub fn new(source: String, r#type: TypeEnum, file_hashes: Vec<String>, message: String) -> PassportElementErrorTranslationFiles {
        PassportElementErrorTranslationFiles {
            source,
            r#type,
            file_hashes,
            message,
        }
    }
}
/// Type of element of the user's Telegram Passport which has the issue, one of “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
    #[serde(rename = "utility_bill")]
    UtilityBill,
    #[serde(rename = "bank_statement")]
    BankStatement,
    #[serde(rename = "rental_agreement")]
    RentalAgreement,
    #[serde(rename = "passport_registration")]
    PassportRegistration,
    #[serde(rename = "temporary_registration")]
    TemporaryRegistration,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::Passport
    }
}

