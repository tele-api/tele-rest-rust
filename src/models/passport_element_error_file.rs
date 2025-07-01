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

/// PassportElementErrorFile : Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorFile {
    /// Error source, must be *file*
    #[serde(rename = "source")]
    pub source: String,
    /// The section of the user's Telegram Passport which has the issue, one of “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”
    #[serde(rename = "type")]
    pub r#type: TypeEnum,
    /// Base64-encoded file hash
    #[serde(rename = "file_hash")]
    pub file_hash: String,
    /// Error message
    #[serde(rename = "message")]
    pub message: String,
}

impl PassportElementErrorFile {
    /// Represents an issue with a document scan. The error is considered resolved when the file with the document scan changes.
    pub fn new(source: String, r#type: TypeEnum, file_hash: String, message: String) -> PassportElementErrorFile {
        PassportElementErrorFile {
            source,
            r#type,
            file_hash,
            message,
        }
    }
}
/// The section of the user's Telegram Passport which has the issue, one of “utility\\_bill”, “bank\\_statement”, “rental\\_agreement”, “passport\\_registration”, “temporary\\_registration”
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeEnum {
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
        Self::UtilityBill
    }
}

