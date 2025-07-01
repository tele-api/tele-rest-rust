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

/// ReactionType : This object describes the type of a reaction. Currently, it can be one of  * [ReactionTypeEmoji](https://core.telegram.org/bots/api/#reactiontypeemoji) * [ReactionTypeCustomEmoji](https://core.telegram.org/bots/api/#reactiontypecustomemoji) * [ReactionTypePaid](https://core.telegram.org/bots/api/#reactiontypepaid)
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReactionType {
    /// Type of the reaction, always “paid”
    #[serde(rename = "type")]
    pub r#type: String,
    /// Reaction emoji. Currently, it can be one of \"❤\", \"👍\", \"👎\", \"🔥\", \"🥰\", \"👏\", \"😁\", \"🤔\", \"🤯\", \"😱\", \"🤬\", \"😢\", \"🎉\", \"🤩\", \"🤮\", \"💩\", \"🙏\", \"👌\", \"🕊\", \"🤡\", \"🥱\", \"🥴\", \"😍\", \"🐳\", \"❤‍🔥\", \"🌚\", \"🌭\", \"💯\", \"🤣\", \"⚡\", \"🍌\", \"🏆\", \"💔\", \"🤨\", \"😐\", \"🍓\", \"🍾\", \"💋\", \"🖕\", \"😈\", \"😴\", \"😭\", \"🤓\", \"👻\", \"👨‍💻\", \"👀\", \"🎃\", \"🙈\", \"😇\", \"😨\", \"🤝\", \"✍\", \"🤗\", \"🫡\", \"🎅\", \"🎄\", \"☃\", \"💅\", \"🤪\", \"🗿\", \"🆒\", \"💘\", \"🙉\", \"🦄\", \"😘\", \"💊\", \"🙊\", \"😎\", \"👾\", \"🤷‍♂\", \"🤷\", \"🤷‍♀\", \"😡\"
    #[serde(rename = "emoji")]
    pub emoji: EmojiEnum,
    /// Custom emoji identifier
    #[serde(rename = "custom_emoji_id")]
    pub custom_emoji_id: String,
}

impl ReactionType {
    /// This object describes the type of a reaction. Currently, it can be one of  * [ReactionTypeEmoji](https://core.telegram.org/bots/api/#reactiontypeemoji) * [ReactionTypeCustomEmoji](https://core.telegram.org/bots/api/#reactiontypecustomemoji) * [ReactionTypePaid](https://core.telegram.org/bots/api/#reactiontypepaid)
    pub fn new(r#type: String, emoji: EmojiEnum, custom_emoji_id: String) -> ReactionType {
        ReactionType {
            r#type,
            emoji,
            custom_emoji_id,
        }
    }
}
/// Reaction emoji. Currently, it can be one of \"❤\", \"👍\", \"👎\", \"🔥\", \"🥰\", \"👏\", \"😁\", \"🤔\", \"🤯\", \"😱\", \"🤬\", \"😢\", \"🎉\", \"🤩\", \"🤮\", \"💩\", \"🙏\", \"👌\", \"🕊\", \"🤡\", \"🥱\", \"🥴\", \"😍\", \"🐳\", \"❤‍🔥\", \"🌚\", \"🌭\", \"💯\", \"🤣\", \"⚡\", \"🍌\", \"🏆\", \"💔\", \"🤨\", \"😐\", \"🍓\", \"🍾\", \"💋\", \"🖕\", \"😈\", \"😴\", \"😭\", \"🤓\", \"👻\", \"👨‍💻\", \"👀\", \"🎃\", \"🙈\", \"😇\", \"😨\", \"🤝\", \"✍\", \"🤗\", \"🫡\", \"🎅\", \"🎄\", \"☃\", \"💅\", \"🤪\", \"🗿\", \"🆒\", \"💘\", \"🙉\", \"🦄\", \"😘\", \"💊\", \"🙊\", \"😎\", \"👾\", \"🤷‍♂\", \"🤷\", \"🤷‍♀\", \"😡\"
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EmojiEnum {
    #[serde(rename = "❤")]
    ,
    #[serde(rename = "👍")]
    2,
    #[serde(rename = "👎")]
    3,
    #[serde(rename = "🔥")]
    4,
    #[serde(rename = "🥰")]
    5,
    #[serde(rename = "👏")]
    6,
    #[serde(rename = "😁")]
    7,
    #[serde(rename = "🤔")]
    8,
    #[serde(rename = "🤯")]
    9,
    #[serde(rename = "😱")]
    10,
    #[serde(rename = "🤬")]
    11,
    #[serde(rename = "😢")]
    12,
    #[serde(rename = "🎉")]
    13,
    #[serde(rename = "🤩")]
    14,
    #[serde(rename = "🤮")]
    15,
    #[serde(rename = "💩")]
    16,
    #[serde(rename = "🙏")]
    17,
    #[serde(rename = "👌")]
    18,
    #[serde(rename = "🕊")]
    19,
    #[serde(rename = "🤡")]
    20,
    #[serde(rename = "🥱")]
    21,
    #[serde(rename = "🥴")]
    22,
    #[serde(rename = "😍")]
    23,
    #[serde(rename = "🐳")]
    24,
    #[serde(rename = "❤‍🔥")]
    25,
    #[serde(rename = "🌚")]
    26,
    #[serde(rename = "🌭")]
    27,
    #[serde(rename = "💯")]
    28,
    #[serde(rename = "🤣")]
    29,
    #[serde(rename = "⚡")]
    30,
    #[serde(rename = "🍌")]
    31,
    #[serde(rename = "🏆")]
    32,
    #[serde(rename = "💔")]
    33,
    #[serde(rename = "🤨")]
    34,
    #[serde(rename = "😐")]
    35,
    #[serde(rename = "🍓")]
    36,
    #[serde(rename = "🍾")]
    37,
    #[serde(rename = "💋")]
    38,
    #[serde(rename = "🖕")]
    39,
    #[serde(rename = "😈")]
    40,
    #[serde(rename = "😴")]
    41,
    #[serde(rename = "😭")]
    42,
    #[serde(rename = "🤓")]
    43,
    #[serde(rename = "👻")]
    44,
    #[serde(rename = "👨‍💻")]
    45,
    #[serde(rename = "👀")]
    46,
    #[serde(rename = "🎃")]
    47,
    #[serde(rename = "🙈")]
    48,
    #[serde(rename = "😇")]
    49,
    #[serde(rename = "😨")]
    50,
    #[serde(rename = "🤝")]
    51,
    #[serde(rename = "✍")]
    52,
    #[serde(rename = "🤗")]
    53,
    #[serde(rename = "🫡")]
    54,
    #[serde(rename = "🎅")]
    55,
    #[serde(rename = "🎄")]
    56,
    #[serde(rename = "☃")]
    57,
    #[serde(rename = "💅")]
    58,
    #[serde(rename = "🤪")]
    59,
    #[serde(rename = "🗿")]
    60,
    #[serde(rename = "🆒")]
    61,
    #[serde(rename = "💘")]
    62,
    #[serde(rename = "🙉")]
    63,
    #[serde(rename = "🦄")]
    64,
    #[serde(rename = "😘")]
    65,
    #[serde(rename = "💊")]
    66,
    #[serde(rename = "🙊")]
    67,
    #[serde(rename = "😎")]
    68,
    #[serde(rename = "👾")]
    69,
    #[serde(rename = "🤷‍♂")]
    70,
    #[serde(rename = "🤷")]
    71,
    #[serde(rename = "🤷‍♀")]
    72,
    #[serde(rename = "😡")]
    73,
}

impl Default for EmojiEnum {
    fn default() -> EmojiEnum {
        Self::
    }
}

