use crate::{
    errors::{WhatsAppError, WhatsAppResult},
    client::validation::validate_phone_number,
};
use serde::Serialize;
use chrono::NaiveDate;

/// A contact message that can be sent via WhatsApp
/// 
/// Contact messages display rich contact information that users can save
/// to their phone's contact list or use to start a new conversation.
#[derive(Debug, Clone, Serialize)]
pub struct ContactMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Contact information
    contacts: Vec<ContactInfo>,
}

/// Complete contact information structure
#[derive(Debug, Clone, Serialize)]
struct ContactInfo {
    /// Physical addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    addresses: Option<Vec<ContactAddress>>,
    /// Birthday in YYYY-MM-DD format
    #[serde(skip_serializing_if = "Option::is_none")]
    birthday: Option<String>,
    /// Email addresses
    #[serde(skip_serializing_if = "Option::is_none")]
    emails: Option<Vec<ContactEmail>>,
    /// Name information (required)
    name: ContactName,
    /// Organization information
    #[serde(skip_serializing_if = "Option::is_none")]
    org: Option<ContactOrganization>,
    /// Phone numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    phones: Option<Vec<ContactPhone>>,
    /// Website URLs
    #[serde(skip_serializing_if = "Option::is_none")]
    urls: Option<Vec<ContactUrl>>,
}

/// Contact address information
#[derive(Debug, Clone, Serialize)]
pub struct ContactAddress {
    /// Street address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// City
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// State or province
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// ZIP or postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Country name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// ISO country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Address type (Home, Work, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub address_type: Option<String>,
}

/// Contact email information
#[derive(Debug, Clone, Serialize)]
pub struct ContactEmail {
    /// Email address
    pub email: String,
    /// Email type (Work, Personal, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub email_type: Option<String>,
}

/// Contact name information
#[derive(Debug, Clone, Serialize)]
pub struct ContactName {
    /// Full formatted name (required)
    pub formatted_name: String,
    /// First name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Middle name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// Name suffix (Jr., Sr., etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Name prefix (Mr., Ms., Dr., etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// Contact organization information
#[derive(Debug, Clone, Serialize)]
pub struct ContactOrganization {
    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Department
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// Job title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Contact phone information
#[derive(Debug, Clone, Serialize)]
pub struct ContactPhone {
    /// Phone number
    pub phone: String,
    /// WhatsApp ID (if the contact is on WhatsApp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wa_id: Option<String>,
    /// Phone type (Mobile, Home, Work, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub phone_type: Option<String>,
}

/// Contact URL information
#[derive(Debug, Clone, Serialize)]
pub struct ContactUrl {
    /// Website URL
    pub url: String,
    /// URL type (Company, Personal, etc.)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub url_type: Option<String>,
}

impl ContactMessage {
    /// Create a new contact message with basic name information
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `formatted_name` - Full name as it should be displayed
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::ContactMessage;
    /// let message = ContactMessage::new("+1234567890", "John Doe")?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn new(to: &str, formatted_name: &str) -> WhatsAppResult<Self> {
        validate_phone_number(to)?;
        
        if formatted_name.is_empty() {
            return Err(WhatsAppError::InvalidMessageContent(
                "Contact formatted name cannot be empty".to_string()
            ));
        }
        
        let contact_info = ContactInfo {
            addresses: None,
            birthday: None,
            emails: None,
            name: ContactName {
                formatted_name: formatted_name.to_string(),
                first_name: None,
                last_name: None,
                middle_name: None,
                suffix: None,
                prefix: None,
            },
            org: None,
            phones: None,
            urls: None,
        };
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            to: to.to_string(),
            message_type: "contacts".to_string(),
            contacts: vec![contact_info],
        })
    }
    
    /// Add detailed name information
    pub fn with_name_details(
        mut self,
        first_name: Option<String>,
        last_name: Option<String>,
        middle_name: Option<String>,
        prefix: Option<String>,
        suffix: Option<String>,
    ) -> Self {
        if let Some(contact) = self.contacts.first_mut() {
            contact.name.first_name = first_name;
            contact.name.last_name = last_name;
            contact.name.middle_name = middle_name;
            contact.name.prefix = prefix;
            contact.name.suffix = suffix;
        }
        self
    }
    
    /// Add phone numbers to the contact
    pub fn with_phones(mut self, phones: Vec<ContactPhone>) -> Self {
        if let Some(contact) = self.contacts.first_mut() {
            contact.phones = Some(phones);
        }
        self
    }
    
    /// Add email addresses to the contact
    pub fn with_emails(mut self, emails: Vec<ContactEmail>) -> Self {
        if let Some(contact) = self.contacts.first_mut() {
            contact.emails = Some(emails);
        }
        self
    }
    
    /// Add addresses to the contact
    pub fn with_addresses(mut self, addresses: Vec<ContactAddress>) -> Self {
        if let Some(contact) = self.contacts.first_mut() {
            contact.addresses = Some(addresses);
        }
        self
    }
    
    /// Add organization information
    pub fn with_organization(mut self, org: ContactOrganization) -> Self {
        if let Some(contact) = self.contacts.first_mut() {
            contact.org = Some(org);
        }
        self
    }
    
    /// Add website URLs
    pub fn with_urls(mut self, urls: Vec<ContactUrl>) -> Self {
        if let Some(contact) = self.contacts.first_mut() {
            contact.urls = Some(urls);
        }
        self
    }
    
    /// Add birthday information
    pub fn with_birthday(mut self, birthday: &str) -> WhatsAppResult<Self> {
        // Validate birthday format (YYYY-MM-DD)
        if !birthday.is_empty() && !is_valid_date_format(birthday) {
            return Err(WhatsAppError::InvalidMessageContent(
                "Birthday must be in YYYY-MM-DD format".to_string()
            ));
        }
        
        if let Some(contact) = self.contacts.first_mut() {
            contact.birthday = Some(birthday.to_string());
        }
        Ok(self)
    }
    
    /// Get the recipient phone number
    pub fn recipient(&self) -> &str {
        &self.to
    }
    
    /// Get the contact's formatted name
    pub fn contact_name(&self) -> Option<&str> {
        self.contacts.first().map(|c| c.name.formatted_name.as_str())
    }
}

/// Helper function to validate date format (YYYY-MM-DD)
fn is_valid_date_format(date: &str) -> bool {
    use regex::Regex;
    use std::sync::OnceLock;
    
    static DATE_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = DATE_REGEX.get_or_init(|| {
        Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("Invalid date regex")
    });

    // Check the MM and DD ranges
    if regex.is_match(date) {
        return NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok();
    }
    false 
}

impl ContactPhone {
    /// Create a new contact phone
    pub fn new(phone: String) -> Self {
        Self {
            phone,
            wa_id: None,
            phone_type: None,
        }
    }
    
    /// Create a contact phone with WhatsApp ID
    pub fn with_whatsapp(phone: String, wa_id: String) -> Self {
        Self {
            phone,
            wa_id: Some(wa_id),
            phone_type: None,
        }
    }
    
    /// Set the phone type
    pub fn with_type(mut self, phone_type: String) -> Self {
        self.phone_type = Some(phone_type);
        self
    }
}

impl ContactEmail {
    /// Create a new contact email
    pub fn new(email: String) -> Self {
        Self {
            email,
            email_type: None,
        }
    }
    
    /// Create a contact email with type
    pub fn with_type(email: String, email_type: String) -> Self {
        Self {
            email,
            email_type: Some(email_type),
        }
    }
}

impl ContactUrl {
    /// Create a new contact URL
    pub fn new(url: String) -> Self {
        Self {
            url,
            url_type: None,
        }
    }
    
    /// Create a contact URL with type
    pub fn with_type(url: String, url_type: String) -> Self {
        Self {
            url,
            url_type: Some(url_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_basic_contact_message() {
        let message = ContactMessage::new("+1234567890", "John Doe").unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.contact_name(), Some("John Doe"));
    }
    
    #[test]
    fn test_contact_with_details() {
        let phones = vec![
            ContactPhone::new("+1234567890".to_string()).with_type("Mobile".to_string()),
        ];
        
        let emails = vec![
            ContactEmail::with_type("john@example.com".to_string(), "Work".to_string()),
        ];
        
        let message = ContactMessage::new("+1234567890", "John Doe")
            .unwrap()
            .with_name_details(
                Some("John".to_string()),
                Some("Doe".to_string()),
                None,
                Some("Mr.".to_string()),
                None,
            )
            .with_phones(phones)
            .with_emails(emails);
        
        assert_eq!(message.contact_name(), Some("John Doe"));
    }
    
    #[test]
    fn test_contact_message_serialization() {
        let message = ContactMessage::new("+1234567890", "John Doe").unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "contacts");
        assert_eq!(json["contacts"][0]["name"]["formatted_name"], "John Doe");
    }
    
    #[test]
    fn test_invalid_birthday_format() {
        let result = ContactMessage::new("+1234567890", "John Doe")
            .unwrap()
            .with_birthday("invalid-date");
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_valid_birthday_format() {
        let result = ContactMessage::new("+1234567890", "John Doe")
            .unwrap()
            .with_birthday("1990-05-15");
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_date_format_validation() {
        assert!(is_valid_date_format("2023-12-25"));
        assert!(is_valid_date_format("1990-01-01"));
        assert!(!is_valid_date_format("2023-13-25")); // Invalid month
        assert!(!is_valid_date_format("23-12-25")); // Wrong year format
        assert!(!is_valid_date_format("2023/12/25")); // Wrong separator
        assert!(!is_valid_date_format("invalid"));
    }
}
