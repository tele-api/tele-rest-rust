//! # Telegram Bot API - REST API Client
//! 
//! The Bot API is an HTTP-based interface created for developers keen on building bots for Telegram. To learn how to create and set up a bot, please consult our Introduction to Bots and Bot FAQ.
//! 
//! ## Metadata
//!   
//! - **Copyright**: Copyright (c) 2025 Qntx
//! - **Author**: ΣX <gitctrlx@gmail.com>
//! - **Version**: 9.0.0
//! - **Modified**: 2025-07-02T07:03:16.715318580Z[Etc/UTC]
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

/// StoryAreaPosition : Describes the position of a clickable area within a story.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryAreaPosition {
    /// The abscissa of the area's center, as a percentage of the media width
    #[serde(rename = "x_percentage")]
    pub x_percentage: f64,
    /// The ordinate of the area's center, as a percentage of the media height
    #[serde(rename = "y_percentage")]
    pub y_percentage: f64,
    /// The width of the area's rectangle, as a percentage of the media width
    #[serde(rename = "width_percentage")]
    pub width_percentage: f64,
    /// The height of the area's rectangle, as a percentage of the media height
    #[serde(rename = "height_percentage")]
    pub height_percentage: f64,
    /// The clockwise rotation angle of the rectangle, in degrees; 0-360
    #[serde(rename = "rotation_angle")]
    pub rotation_angle: f64,
    /// The radius of the rectangle corner rounding, as a percentage of the media width
    #[serde(rename = "corner_radius_percentage")]
    pub corner_radius_percentage: f64,
}

impl StoryAreaPosition {
    /// Describes the position of a clickable area within a story.
    pub fn new(x_percentage: f64, y_percentage: f64, width_percentage: f64, height_percentage: f64, rotation_angle: f64, corner_radius_percentage: f64) -> StoryAreaPosition {
        StoryAreaPosition {
            x_percentage,
            y_percentage,
            width_percentage,
            height_percentage,
            rotation_angle,
            corner_radius_percentage,
        }
    }
}

