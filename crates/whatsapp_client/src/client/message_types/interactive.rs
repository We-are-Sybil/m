use crate::{
    errors::{WhatsAppError, WhatsAppResult},
    client::validation::{
        validate_phone_number, validate_button, validate_list_section,
        validate_header_text, validate_footer_text, validate_text_message, validate_url
    },
};
use serde::Serialize;

/// An interactive message that can be sent via WhatsApp
/// 
/// Interactive messages provide structured ways for users to respond,
/// including buttons, lists, call-to-action URLs, and location requests.
#[derive(Debug, Clone, Serialize)]
pub struct InteractiveMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Interactive content configuration
    interactive: InteractiveContent,
}

/// Interactive message content structure
/// 
/// This contains the type of interaction and all associated content.
/// Different interaction types have different required fields.
#[derive(Debug, Clone, Serialize)]
struct InteractiveContent {
    /// Type of interactive message
    #[serde(rename = "type")]
    interactive_type: String,
    /// Optional header (text, image, video, or document)
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<InteractiveHeader>,
    /// Message body text
    body: InteractiveBody,
    /// Optional footer text
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<InteractiveFooter>,
    /// Action configuration (buttons, list, URL, etc.)
    action: InteractiveAction,
}

/// Header for interactive messages
#[derive(Debug, Clone, Serialize)]
struct InteractiveHeader {
    /// Header type (text, image, video, document)
    #[serde(rename = "type")]
    header_type: String,
    /// Text content for text headers
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    /// Image configuration for image headers
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<MediaReference>,
    /// Video configuration for video headers
    #[serde(skip_serializing_if = "Option::is_none")]
    video: Option<MediaReference>,
    /// Document configuration for document headers
    #[serde(skip_serializing_if = "Option::is_none")]
    document: Option<MediaReference>,
}

/// Media reference for headers
#[derive(Debug, Clone, Serialize)]
struct MediaReference {
    /// Media ID for uploaded media
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// URL for hosted media
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,
}

/// Body text for interactive messages
#[derive(Debug, Clone, Serialize)]
struct InteractiveBody {
    /// Body text content
    text: String,
}

/// Footer text for interactive messages
#[derive(Debug, Clone, Serialize)]
struct InteractiveFooter {
    /// Footer text content
    text: String,
}

/// Action configuration for interactive messages
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
enum InteractiveAction {
    /// Button actions (up to 3 buttons)
    Buttons {
        buttons: Vec<InteractiveButton>,
    },
    /// List actions (single list with sections)
    List {
        button: String,
        sections: Vec<InteractiveListSection>,
    },
    /// Call-to-action URL button
    CtaUrl {
        name: String,
        parameters: CtaUrlParameters,
    },
    /// Location request
    LocationRequest {
        name: String,
    },
}

/// Individual button for button-type interactive messages
#[derive(Debug, Clone, Serialize)]
struct InteractiveButton {
    /// Always "reply" for reply buttons
    #[serde(rename = "type")]
    button_type: String,
    /// Reply button configuration
    reply: ButtonReply,
}

/// Button reply configuration
#[derive(Debug, Clone, Serialize)]
struct ButtonReply {
    /// Unique button identifier
    id: String,
    /// Button display text
    title: String,
}

/// Section for list-type interactive messages
#[derive(Debug, Clone, Serialize)]
struct InteractiveListSection {
    /// Section title
    title: String,
    /// Rows in this section
    rows: Vec<InteractiveListRow>,
}

/// Row in a list section
#[derive(Debug, Clone, Serialize)]
struct InteractiveListRow {
    /// Unique row identifier
    id: String,
    /// Row title
    title: String,
    /// Optional row description
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

/// Parameters for call-to-action URL buttons
#[derive(Debug, Clone, Serialize)]
struct CtaUrlParameters {
    /// Button display text
    display_text: String,
    /// URL to open when button is tapped
    url: String,
}

impl InteractiveMessage {
    /// Create a new interactive message with reply buttons
    /// 
    /// Reply buttons allow users to quickly respond with predefined options.
    /// Up to 3 buttons are supported per message.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `body_text` - Main message text
    /// * `buttons` - List of (id, title) pairs for buttons (max 3)
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::InteractiveMessage;
    /// let buttons = vec![
    ///     ("yes".to_string(), "Yes".to_string()),
    ///     ("no".to_string(), "No".to_string()),
    /// ];
    /// let message = InteractiveMessage::with_buttons(
    ///     "+1234567890",
    ///     "Do you want to continue?",
    ///     buttons
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_buttons(
        to: &str,
        body_text: &str,
        buttons: Vec<(String, String)>,
    ) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_text_message(body_text)?;
        
        if buttons.is_empty() || buttons.len() > 3 {
            return Err(WhatsAppError::InvalidMessageContent(
                "Interactive messages must have 1-3 buttons".to_string()
            ));
        }
        
        // Validate and convert buttons
        let interactive_buttons: Result<Vec<InteractiveButton>, WhatsAppError> = buttons
            .into_iter()
            .map(|(id, title)| {
                validate_button(&id, &title)?;
                Ok(InteractiveButton {
                    button_type: "reply".to_string(),
                    reply: ButtonReply { id, title },
                })
            })
            .collect();
        
        let interactive_buttons = interactive_buttons?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: InteractiveContent {
                interactive_type: "button".to_string(),
                header: None,
                body: InteractiveBody {
                    text: body_text.to_string(),
                },
                footer: None,
                action: InteractiveAction::Buttons {
                    buttons: interactive_buttons,
                },
            },
        })
    }
    
    /// Create a new interactive message with a list
    /// 
    /// List messages provide a menu-style interface where users can
    /// select from organized options. Lists support up to 10 sections
    /// with up to 10 total rows across all sections.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `body_text` - Main message text
    /// * `button_text` - Text for the button that opens the list
    /// * `sections` - List sections with (title, rows) where rows are (id, title, description)
    pub fn with_list(
        to: &str,
        body_text: &str,
        button_text: &str,
        sections: Vec<(String, Vec<(String, String, Option<String>)>)>,
    ) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_text_message(body_text)?;
        validate_button("list_button", button_text)?;
        
        if sections.is_empty() || sections.len() > 10 {
            return Err(WhatsAppError::InvalidMessageContent(
                "List messages must have 1-10 sections".to_string()
            ));
        }
        
        // Count total rows across all sections
        let total_rows: usize = sections.iter().map(|(_, rows)| rows.len()).sum();
        if total_rows > 10 {
            return Err(WhatsAppError::InvalidMessageContent(
                format!("List messages can have at most 10 total rows, got {}", total_rows)
            ));
        }
        
        // Validate and convert sections
        let interactive_sections: Result<Vec<InteractiveListSection>, WhatsAppError> = sections
            .into_iter()
            .map(|(title, rows)| {
                validate_list_section(&title, &rows)?;
                
                let interactive_rows: Vec<InteractiveListRow> = rows
                    .into_iter()
                    .map(|(id, row_title, description)| InteractiveListRow {
                        id,
                        title: row_title,
                        description,
                    })
                    .collect();
                
                Ok(InteractiveListSection {
                    title,
                    rows: interactive_rows,
                })
            })
            .collect();
        
        let interactive_sections = interactive_sections?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: InteractiveContent {
                interactive_type: "list".to_string(),
                header: None,
                body: InteractiveBody {
                    text: body_text.to_string(),
                },
                footer: None,
                action: InteractiveAction::List {
                    button: button_text.to_string(),
                    sections: interactive_sections,
                },
            },
        })
    }
    
    /// Create a call-to-action URL button message
    /// 
    /// CTA URL buttons allow users to visit a website by tapping a button.
    /// This is useful for directing users to external resources without
    /// showing a raw URL in the message.
    pub fn with_cta_url(
        to: &str,
        body_text: &str,
        button_text: &str,
        url: &str,
    ) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_text_message(body_text)?;
        validate_button("cta_button", button_text)?;
        validate_url(url)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: InteractiveContent {
                interactive_type: "cta_url".to_string(),
                header: None,
                body: InteractiveBody {
                    text: body_text.to_string(),
                },
                footer: None,
                action: InteractiveAction::CtaUrl {
                    name: "cta_url".to_string(),
                    parameters: CtaUrlParameters {
                        display_text: button_text.to_string(),
                        url: url.to_string(),
                    },
                },
            },
        })
    }
    
    /// Create a location request message
    /// 
    /// Location request messages prompt users to share their current location.
    /// This is useful for location-based services or delivery applications.
    pub fn request_location(to: &str, body_text: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_text_message(body_text)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "interactive".to_string(),
            interactive: InteractiveContent {
                interactive_type: "location_request_message".to_string(),
                header: None,
                body: InteractiveBody {
                    text: body_text.to_string(),
                },
                footer: None,
                action: InteractiveAction::LocationRequest {
                    name: "send_location".to_string(),
                },
            },
        })
    }
    
    /// Add a text header to the message
    pub fn with_text_header(mut self, header_text: &str) -> WhatsAppResult<Self> {
        validate_header_text(header_text)?;
        
        self.interactive.header = Some(InteractiveHeader {
            header_type: "text".to_string(),
            text: Some(header_text.to_string()),
            image: None,
            video: None,
            document: None,
        });
        
        Ok(self)
    }
    
    /// Add a footer to the message
    pub fn with_footer(mut self, footer_text: &str) -> WhatsAppResult<Self> {
        validate_footer_text(footer_text)?;
        
        self.interactive.footer = Some(InteractiveFooter {
            text: footer_text.to_string(),
        });
        
        Ok(self)
    }
    
    /// Get the recipient phone number
    pub fn recipient(&self) -> &str {
        &self.to
    }
    
    /// Get the message body text
    pub fn body_text(&self) -> &str {
        &self.interactive.body.text
    }
    
    /// Get the interaction type
    pub fn interaction_type(&self) -> &str {
        &self.interactive.interactive_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_interactive_message_with_buttons() {
        let buttons = vec![
            ("yes".to_string(), "Yes".to_string()),
            ("no".to_string(), "No".to_string()),
        ];
        
        let message = InteractiveMessage::with_buttons(
            "+1234567890",
            "Do you want to continue?",
            buttons
        ).unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.body_text(), "Do you want to continue?");
        assert_eq!(message.interaction_type(), "button");
    }
    
    #[test]
    fn test_interactive_message_with_list() {
        let sections = vec![
            ("Options".to_string(), vec![
                ("opt1".to_string(), "Option 1".to_string(), Some("First option".to_string())),
                ("opt2".to_string(), "Option 2".to_string(), None),
            ]),
        ];
        
        let message = InteractiveMessage::with_list(
            "+1234567890",
            "Choose an option:",
            "Select",
            sections
        ).unwrap();
        
        assert_eq!(message.interaction_type(), "list");
    }
    
    #[test]
    fn test_interactive_message_with_cta_url() {
        let message = InteractiveMessage::with_cta_url(
            "+1234567890",
            "Visit our website for more info",
            "Visit Website",
            "https://example.com"
        ).unwrap();
        
        assert_eq!(message.interaction_type(), "cta_url");
    }
    
    #[test]
    fn test_location_request_message() {
        let message = InteractiveMessage::request_location(
            "+1234567890",
            "Please share your location for delivery"
        ).unwrap();
        
        assert_eq!(message.interaction_type(), "location_request_message");
    }
    
    #[test]
    fn test_message_with_header_and_footer() {
        let buttons = vec![("ok".to_string(), "OK".to_string())];
        
        let message = InteractiveMessage::with_buttons(
            "+1234567890",
            "Main message",
            buttons
        ).unwrap()
        .with_text_header("Header Text").unwrap()
        .with_footer("Footer Text").unwrap();
        
        assert_eq!(message.body_text(), "Main message");
    }
    
    #[test]
    fn test_too_many_buttons() {
        let buttons = vec![
            ("1".to_string(), "One".to_string()),
            ("2".to_string(), "Two".to_string()),
            ("3".to_string(), "Three".to_string()),
            ("4".to_string(), "Four".to_string()), // Too many
        ];
        
        let result = InteractiveMessage::with_buttons(
            "+1234567890",
            "Choose:",
            buttons
        );
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_interactive_message_serialization() {
        let buttons = vec![("yes".to_string(), "Yes".to_string())];
        
        let message = InteractiveMessage::with_buttons(
            "+1234567890",
            "Continue?",
            buttons
        ).unwrap();
        
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["type"], "interactive");
        assert_eq!(json["interactive"]["type"], "button");
        assert_eq!(json["interactive"]["body"]["text"], "Continue?");
        assert_eq!(json["interactive"]["action"]["buttons"][0]["reply"]["id"], "yes");
        assert_eq!(json["interactive"]["action"]["buttons"][0]["reply"]["title"], "Yes");
    }
}
