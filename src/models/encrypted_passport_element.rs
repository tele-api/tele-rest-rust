//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-01T14:36:16.092164073Z[Etc/UTC]
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

/// EncryptedPassportElement : Describes documents or other Telegram Passport elements shared with the bot by the user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    /// Element type. One of “personal\\_details”, “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “address”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”, “phone\\_number”, “email”.
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// *Optional*. Base64-encoded encrypted Telegram Passport element data provided by the user; available only for “personal\\_details”, “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport” and “address” types. Can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// *Optional*. User's verified phone number; available only for “phone\\_number” type
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// *Optional*. User's verified email address; available only for “email” type
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// *Optional*. Array of encrypted files with documents provided by the user; available only for “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration” and “temporary\\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::PassportFile>>,
    #[serde(rename = "front_side", skip_serializing_if = "Option::is_none")]
    pub front_side: Option<Box<models::PassportFile>>,
    #[serde(rename = "reverse_side", skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<Box<models::PassportFile>>,
    #[serde(rename = "selfie", skip_serializing_if = "Option::is_none")]
    pub selfie: Option<Box<models::PassportFile>>,
    /// *Optional*. Array of encrypted files with translated versions of documents provided by the user; available if requested for “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration” and “temporary\\_registration” types. Files can be decrypted and verified using the accompanying [EncryptedCredentials](https://core.telegram.org/bots/api/#encryptedcredentials).
    #[serde(rename = "translation", skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<models::PassportFile>>,
    /// Base64-encoded element hash for using in [PassportElementErrorUnspecified](https://core.telegram.org/bots/api/#passportelementerrorunspecified)
    #[serde(rename = "hash")]
    pub hash: String,
}

impl EncryptedPassportElement {
    /// Describes documents or other Telegram Passport elements shared with the bot by the user.
    pub fn new(r#type: TypeEnum, hash: String) -> EncryptedPassportElement {
        EncryptedPassportElement {
            r#type,
            data: None,
            phone_number: None,
            email: None,
            files: None,
            front_side: None,
            reverse_side: None,
            selfie: None,
            translation: None,
            hash,
        }
    }
}
/// Element type. One of “personal\\_details”, “passport”, “driver\\_license”, “identity\\_card”, “internal\\_passport”, “address”, “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”, “phone\\_number”, “email”.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
    #[serde(rename = "personal_details")]
    PersonalDetails,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "driver_license")]
    DriverLicense,
    #[serde(rename = "identity_card")]
    IdentityCard,
    #[serde(rename = "internal_passport")]
    InternalPassport,
    #[serde(rename = "address")]
    Address,
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
    #[serde(rename = "phone_number")]
    PhoneNumber,
    #[serde(rename = "email")]
    Email,
}

impl Default for TypeEnum {
    fn default() -> TypeEnum {
        Self::PersonalDetails
    }
}

