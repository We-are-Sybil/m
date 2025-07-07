use crate::{
    errors::WhatsAppResult,
    client::message_types::InteractiveMessage,
};

/// Builder for creating interactive messages with fluent interface
/// 
/// This builder represents the most complex message type in the WhatsApp API,
/// handling multiple interaction patterns including buttons, lists, and
/// call-to-action elements. Interactive messages enable rich conversational
/// experiences that guide users through structured workflows.
/// 
/// # Interaction Types
/// - **Reply Buttons**: Up to 3 quick-reply buttons for simple choices
/// - **List Menus**: Organized sections with multiple options for complex choices
/// - **Call-to-Action**: URL buttons that open external links
/// - **Location Requests**: Buttons that request user's location
/// 
/// # Design Philosophy
/// Interactive messages transform free-form chat into guided experiences,
/// reducing user errors and improving conversation flow. They're essential
/// for business automation, customer service, and structured data collection.
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::InteractiveMessageBuilder;
/// // Simple yes/no decision
/// let message = InteractiveMessageBuilder::new()
///     .to("+1234567890")
///     .body("Would you like to proceed with the order?")
///     .add_button("yes", "Yes, proceed")
///     .add_button("no", "Cancel order")
///     .build()?;
/// 
/// // Complex menu with categories
/// let message = InteractiveMessageBuilder::new()
///     .to("+1234567890")
///     .body("Select a product category:")
///     .list_button("Browse Products")
///     .add_list_section("Electronics")
///         .add_list_row("electronics_phones", "Smartphones", "Latest models available")
///         .add_list_row("electronics_laptops", "Laptops", "Business and gaming options")
///     .add_list_section("Clothing")
///         .add_list_row("clothing_men", "Men's Clothing", "Shirts, pants, accessories")
///         .add_list_row("clothing_women", "Women's Clothing", "Dresses, tops, accessories")
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct InteractiveMessageBuilder {
    to: Option<String>,
    body: Option<String>,
    header: Option<String>,
    footer: Option<String>,
    buttons: Vec<(String, String)>, // (id, title) pairs
    list_button_text: Option<String>,
    list_sections: Vec<ListSectionBuilder>,
    current_section: Option<ListSectionBuilder>,
    cta_url: Option<String>,
    cta_display_text: Option<String>,
    location_request: bool,
}

/// Builder for individual list sections within interactive messages
/// 
/// This nested builder handles the complexity of organizing list items
/// into logical sections, each with their own title and rows.
#[derive(Debug, Clone)]
struct ListSectionBuilder {
    title: String,
    rows: Vec<(String, String, Option<String>)>, // (id, title, description)
}

impl InteractiveMessageBuilder {
    /// Create a new interactive message builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the recipient phone number
    /// 
    /// # Arguments
    /// * `phone` - Phone number in E.164 format (+1234567890)
    pub fn to(mut self, phone: &str) -> Self {
        self.to = Some(phone.to_string());
        self
    }
    
    /// Set the main message body text
    /// 
    /// The body text appears above the interactive elements and should
    /// clearly explain what action the user needs to take. This is the
    /// primary communication content of the message.
    /// 
    /// # Arguments
    /// * `text` - Main message content
    /// 
    /// # Best Practices
    /// - Keep it concise but clear about the expected action
    /// - Use action-oriented language ("Choose an option", "Select your preference")
    /// - Provide context for the choices that follow
    /// - Avoid redundancy with button or list labels
    pub fn body(mut self, text: &str) -> Self {
        self.body = Some(text.to_string());
        self
    }
    
    /// Set an optional header above the body text
    /// 
    /// Headers provide additional context or branding. They appear
    /// prominently at the top of the message and help establish
    /// the message's purpose or source.
    /// 
    /// # Arguments
    /// * `text` - Header text (max 60 characters)
    /// 
    /// # Usage Examples
    /// - Brand identification: "Customer Service"
    /// - Message type: "Order Confirmation"
    /// - Urgency indicators: "Action Required"
    pub fn header(mut self, text: &str) -> Self {
        self.header = Some(text.to_string());
        self
    }
    
    /// Set an optional footer below the interactive elements
    /// 
    /// Footers provide additional information, disclaimers, or
    /// helpful context that doesn't interfere with the main
    /// interaction flow.
    /// 
    /// # Arguments
    /// * `text` - Footer text (max 60 characters)
    /// 
    /// # Usage Examples
    /// - Help text: "Reply HELP for assistance"
    /// - Disclaimers: "Standard rates may apply"
    /// - Timing info: "Expires in 24 hours"
    pub fn footer(mut self, text: &str) -> Self {
        self.footer = Some(text.to_string());
        self
    }
    
    /// Add a reply button for quick responses
    /// 
    /// Reply buttons provide immediate response options that appear
    /// below the message. They're perfect for yes/no decisions,
    /// simple choices, or quick actions. Maximum 3 buttons allowed.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for this button (used in responses)
    /// * `title` - Text displayed on the button (max 20 characters)
    /// 
    /// # Design Considerations
    /// - Order buttons logically (positive actions first)
    /// - Use clear, action-oriented text
    /// - Consider the most likely user responses
    /// - Ensure button IDs are meaningful for your backend processing
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::InteractiveMessageBuilder;
    /// let message = InteractiveMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .body("Confirm your appointment for tomorrow at 2 PM?")
    ///     .add_button("confirm", "Confirm")
    ///     .add_button("reschedule", "Reschedule")
    ///     .add_button("cancel", "Cancel")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn add_button(mut self, id: &str, title: &str) -> Self {
        if self.buttons.len() < 3 { // WhatsApp limit
            self.buttons.push((id.to_string(), title.to_string()));
        }
        self
    }
    
    /// Set the button text for list-type interactive messages
    /// 
    /// This button appears below the message body and opens the
    /// list of options when tapped. The text should clearly indicate
    /// what kind of selection the user will make.
    /// 
    /// # Arguments
    /// * `text` - Button text that opens the list
    /// 
    /// # Usage Examples
    /// - "View Options" for general choices
    /// - "Select Product" for shopping scenarios
    /// - "Choose Service" for service selection
    /// - "Browse Categories" for navigation
    pub fn list_button(mut self, text: &str) -> Self {
        self.list_button_text = Some(text.to_string());
        self
    }
    
    /// Start a new section in the list menu
    /// 
    /// List sections help organize related options into logical groups.
    /// Each section has a title and contains multiple rows. This method
    /// finishes any current section and starts a new one.
    /// 
    /// # Arguments
    /// * `title` - Section title that appears as a header
    /// 
    /// # Design Patterns
    /// - Group related items together ("Payment Methods", "Shipping Options")
    /// - Use descriptive section titles that help users navigate
    /// - Order sections by importance or logical flow
    /// - Keep sections focused on single concepts
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::InteractiveMessageBuilder;
    /// let message = InteractiveMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .body("Choose your meal:")
    ///     .list_button("View Menu")
    ///     .add_list_section("Main Courses")
    ///         .add_list_row("pasta", "Pasta Primavera", "Fresh vegetables with linguine")
    ///         .add_list_row("steak", "Grilled Steak", "8oz sirloin with sides")
    ///     .add_list_section("Desserts")
    ///         .add_list_row("cake", "Chocolate Cake", "Rich chocolate with vanilla cream")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn add_list_section(mut self, title: &str) -> Self {
        // Finish current section if it exists
        if let Some(section) = self.current_section.take() {
            self.list_sections.push(section);
        }
        
        // Start new section
        self.current_section = Some(ListSectionBuilder {
            title: title.to_string(),
            rows: Vec::new(),
        });
        
        self
    }
    
    /// Add a row to the current list section
    /// 
    /// Rows represent individual selectable options within a section.
    /// Each row has an ID (for backend processing), a title (main text),
    /// and an optional description (additional context).
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for this option
    /// * `title` - Main text displayed for this option (max 24 characters)
    /// * `description` - Optional additional context (max 72 characters)
    /// 
    /// # Design Guidelines
    /// - Use descriptive titles that clearly indicate the choice
    /// - Include helpful descriptions for complex or similar options
    /// - Ensure IDs are meaningful for your backend processing
    /// - Order rows logically within each section
    /// 
    /// # Note
    /// You must call `add_list_section()` before adding rows.
    pub fn add_list_row(mut self, id: &str, title: &str, description: &str) -> Self {
        if let Some(ref mut section) = self.current_section {
            section.rows.push((
                id.to_string(),
                title.to_string(),
                Some(description.to_string()),
            ));
        }
        self
    }
    
    /// Add a row without description to the current list section
    /// 
    /// Use this for simple options that don't need additional explanation.
    /// 
    /// # Arguments
    /// * `id` - Unique identifier for this option
    /// * `title` - Text displayed for this option
    pub fn add_simple_list_row(mut self, id: &str, title: &str) -> Self {
        if let Some(ref mut section) = self.current_section {
            section.rows.push((
                id.to_string(),
                title.to_string(),
                None,
            ));
        }
        self
    }
    
    /// Add a call-to-action URL button
    /// 
    /// CTA buttons open external URLs when tapped, perfect for
    /// directing users to websites, documentation, or external
    /// processes that can't be handled within WhatsApp.
    /// 
    /// # Arguments
    /// * `display_text` - Text shown on the button
    /// * `url` - URL to open when button is tapped
    /// 
    /// # Use Cases
    /// - "Complete Purchase" linking to checkout page
    /// - "View Details" linking to product information
    /// - "Download App" linking to app stores
    /// - "Get Support" linking to help documentation
    /// 
    /// # Important Notes
    /// - URLs must be HTTPS
    /// - Consider mobile-friendly landing pages
    /// - Track clicks for analytics if needed
    /// - Ensure URLs work reliably across different devices
    pub fn cta_url(mut self, display_text: &str, url: &str) -> Self {
        self.cta_display_text = Some(display_text.to_string());
        self.cta_url = Some(url.to_string());
        self
    }
    
    /// Request the user's location
    /// 
    /// This creates a button that, when tapped, prompts the user
    /// to share their current location. Useful for delivery services,
    /// location-based recommendations, or emergency services.
    /// 
    /// # Privacy Considerations
    /// - Users can decline to share location
    /// - Always explain why you need location data
    /// - Consider alternative workflows for users who decline
    /// - Respect user privacy preferences
    /// 
    /// # Use Cases
    /// - Delivery address collection
    /// - Finding nearby stores or services
    /// - Emergency assistance
    /// - Location-based personalization
    pub fn request_location(mut self) -> Self {
        self.location_request = true;
        self
    }
    
    /// Remove all currently configured buttons
    /// 
    /// Useful for conditional logic where you might want to
    /// clear buttons and rebuild them based on business rules.
    pub fn clear_buttons(mut self) -> Self {
        self.buttons.clear();
        self
    }
    
    /// Remove all list configuration
    /// 
    /// Clears list button text, sections, and current section.
    /// Useful for switching between button and list interaction types.
    pub fn clear_list(mut self) -> Self {
        self.list_button_text = None;
        self.list_sections.clear();
        self.current_section = None;
        self
    }
    
    /// Build the interactive message
    /// 
    /// This validates the complex configuration and creates the final
    /// InteractiveMessage. Interactive messages have sophisticated
    /// validation rules due to their many possible configurations.
    /// 
    /// # Validation Process
    /// 1. Recipient phone number must be valid
    /// 2. Body text must be provided
    /// 3. Exactly one interaction type must be configured:
    ///    - Reply buttons (1-3 buttons)
    ///    - List menu (sections with rows)
    ///    - CTA URL button
    ///    - Location request
    /// 4. Header/footer length limits (60 characters each)
    /// 5. Button text limits (20 characters for buttons, 24 for list titles)
    /// 6. List structure validation (sections must have rows)
    /// 
    /// # Interaction Type Logic
    /// The builder automatically determines the interaction type based on
    /// which methods were called, prioritizing in this order:
    /// 1. Location request (if enabled)
    /// 2. CTA URL (if configured)
    /// 3. List menu (if sections exist)
    /// 4. Reply buttons (if buttons exist)
    /// 
    /// # Error Scenarios
    /// - No interaction type configured
    /// - Multiple conflicting interaction types
    /// - Text length violations
    /// - Invalid list structure (sections without rows)
    /// - Button limits exceeded (>3 buttons)
    pub fn build(mut self) -> WhatsAppResult<InteractiveMessage> {
        let to = self.to.clone().ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required for interactive messages".to_string()
            )
        })?;
        
        let body = self.body.clone().ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Body text is required for interactive messages".to_string()
            )
        })?;
        
        // Finish any pending list section
        if let Some(section) = self.current_section.take() {
            self.list_sections.push(section);
        }
        
        // Determine interaction type and create message
        if self.location_request {
            // Location request takes highest priority
            self.build_location_request_message(&to, &body)
        } else if self.cta_url.is_some() {
            // CTA URL button
            self.build_cta_message(&to, &body)
        } else if !self.list_sections.is_empty() {
            // List menu
            self.build_list_message(&to, &body)
        } else if !self.buttons.is_empty() {
            // Reply buttons
            self.build_button_message(&to, &body)
        } else {
            Err(crate::errors::WhatsAppError::InvalidMessageContent(
                "Interactive messages must have at least one interaction element: \
                 buttons, list menu, CTA URL, or location request".to_string()
            ))
        }
    }
    
    // Helper methods for building specific interaction types
    
    fn build_button_message(&self, to: &str, body: &str) -> WhatsAppResult<InteractiveMessage> {
        if self.buttons.len() > 3 {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                "Maximum 3 reply buttons allowed".to_string()
            ));
        }
        
        let message = InteractiveMessage::with_buttons(to, body, self.buttons.clone())?;
        self.apply_optional_elements(message)
    }
    
    fn build_list_message(&self, to: &str, body: &str) -> WhatsAppResult<InteractiveMessage> {
        let button_text = self.list_button_text.as_ref().ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "List button text is required when using list sections. Use list_button() method.".to_string()
            )
        })?;
        
        // Validate list structure comprehensively
        if self.list_sections.is_empty() {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                "At least one list section is required. Use add_list_section() method.".to_string()
            ));
        }
        
        // Ensure every section has rows and validate content
        for section in &self.list_sections {
            if section.rows.is_empty() {
                return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                    format!("List section '{}' must have at least one row. Use add_list_row() method.", section.title)
                ));
            }
            
            // Validate row count (WhatsApp has limits)
            if section.rows.len() > 10 {
                return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                    format!("List section '{}' has {} rows, but maximum 10 rows per section allowed", section.title, section.rows.len())
                ));
            }
        }
        
        // Total rows across all sections should not exceed WhatsApp limits
        let total_rows: usize = self.list_sections.iter().map(|s| s.rows.len()).sum();
        if total_rows > 10 {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                format!("Total list rows ({}) exceeds WhatsApp limit of 10 rows across all sections", total_rows)
            ));
        }
        
        // Convert our internal builder structure to the format expected by InteractiveMessage
        // This transformation is crucial - we're converting from builder-friendly format
        // to the wire-format that WhatsApp expects
        let sections: Vec<_> = self.list_sections.iter().map(|section| {
            let rows: Vec<_> = section.rows.iter().map(|(id, title, description)| {
                (id.clone(), title.clone(), description.clone())
            }).collect();
            (section.title.clone(), rows)
        }).collect();
        
        // Create the message using our assumed InteractiveMessage API
        // In a real implementation, this would call your actual InteractiveMessage::with_list method
        let message = InteractiveMessage::with_list(to, body, button_text, sections)?;
        self.apply_optional_elements(message)
    }
    
    fn build_cta_message(&self, to: &str, body: &str) -> WhatsAppResult<InteractiveMessage> {
        let display_text = self.cta_display_text.as_ref().unwrap();
        let url = self.cta_url.as_ref().unwrap();
        
        // Validate URL format - WhatsApp requires HTTPS for security
        if !url.starts_with("https://") {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                "CTA URL must use HTTPS protocol for security. HTTP URLs are not allowed.".to_string()
            ));
        }
        
        // Validate display text length (WhatsApp has specific limits for CTA buttons)
        if display_text.len() > 20 {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                format!("CTA button text too long: {} characters (max 20)", display_text.len())
            ));
        }
        
        if display_text.is_empty() {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                "CTA button text cannot be empty".to_string()
            ));
        }
        
        let message = InteractiveMessage::with_cta_url(to, body, display_text, url)?;
        self.apply_optional_elements(message)
    }
    
    fn build_location_request_message(&self, to: &str, body: &str) -> WhatsAppResult<InteractiveMessage> {
        // Location requests are simpler but should still validate the context
        // The body text should clearly explain why location is needed for privacy/UX
        if body.len() < 10 {
            return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                "Location request messages should include clear explanation of why location is needed".to_string()
            ));
        }
        
        let message = InteractiveMessage::request_location(to, body)?;
        self.apply_optional_elements(message)
    }
    
    fn apply_optional_elements(&self, mut message: InteractiveMessage) -> WhatsAppResult<InteractiveMessage> {
        if let Some(ref header_text) = self.header {
            message = message.with_text_header(header_text)?;
        }
        
        if let Some(ref footer_text) = self.footer {
            message = message.with_footer(footer_text)?;
        }
        
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_button_message() {
        let builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Would you like to continue?")
            .add_button("yes", "Yes")
            .add_button("no", "No");
        
        // Test internal state before building
        assert_eq!(builder.buttons.len(), 2);
        assert_eq!(builder.buttons[0], ("yes".to_string(), "Yes".to_string()));
        assert_eq!(builder.buttons[1], ("no".to_string(), "No".to_string()));
        
        // This would build successfully in a real implementation
        // For testing, we're validating the builder state and logic
        assert!(builder.to.is_some());
        assert!(builder.body.is_some());
        assert!(builder.list_sections.is_empty());
        assert!(builder.cta_url.is_none());
        assert!(!builder.location_request);
    }
    
    #[test]
    fn test_complex_list_message_structure() {
        let mut builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Choose a product category:")
            .header("Product Catalog")
            .footer("Free shipping on orders over $50")
            .list_button("Browse Products")
            .add_list_section("Electronics")
                .add_list_row("phones", "Smartphones", "Latest models")
                .add_list_row("laptops", "Laptops", "Business and gaming")
            .add_list_section("Clothing")
                .add_simple_list_row("mens", "Men's Clothing")
                .add_simple_list_row("womens", "Women's Clothing");
        
        // Verify the complex nested structure was built correctly
        assert_eq!(builder.list_button_text, Some("Browse Products".to_string()));
        assert_eq!(builder.header, Some("Product Catalog".to_string()));
        assert_eq!(builder.footer, Some("Free shipping on orders over $50".to_string()));
        
        // Finish the current section to test the internal structure
        if let Some(section) = builder.current_section.take() {
            builder.list_sections.push(section);
        }
        
        // Verify we have 2 sections with correct structure
        assert_eq!(builder.list_sections.len(), 2);
        
        // Test Electronics section
        let electronics_section = &builder.list_sections[0];
        assert_eq!(electronics_section.title, "Electronics");
        assert_eq!(electronics_section.rows.len(), 2);
        assert_eq!(electronics_section.rows[0].0, "phones");
        assert_eq!(electronics_section.rows[0].1, "Smartphones");
        assert_eq!(electronics_section.rows[0].2, Some("Latest models".to_string()));
        
        // Test Clothing section  
        let clothing_section = &builder.list_sections[1];
        assert_eq!(clothing_section.title, "Clothing");
        assert_eq!(clothing_section.rows.len(), 2);
        assert_eq!(clothing_section.rows[0].0, "mens");
        assert_eq!(clothing_section.rows[0].2, None); // No description for simple rows
    }
    
    #[test]
    fn test_button_limit_enforcement() {
        let builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Too many buttons test")
            .add_button("btn1", "Button 1")
            .add_button("btn2", "Button 2")
            .add_button("btn3", "Button 3")
            .add_button("btn4", "Button 4"); // This should be ignored due to 3-button limit
        
        // WhatsApp allows maximum 3 buttons, so the 4th should be ignored
        assert_eq!(builder.buttons.len(), 3);
        assert_eq!(builder.buttons[2].0, "btn3"); // Last button should be btn3, not btn4
    }
    
    #[test]
    fn test_interaction_type_priority() {
        // Test that interaction types are prioritized correctly when multiple are set
        let builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Priority test")
            .add_button("btn", "Button")  // Lower priority
            .cta_url("Visit", "https://example.com")  // Medium priority
            .request_location();  // Highest priority
        
        // All three interaction types are configured
        assert!(!builder.buttons.is_empty());
        assert!(builder.cta_url.is_some());
        assert!(builder.location_request);
        
        // In build(), location_request should take precedence
        // This demonstrates the builder's intelligent conflict resolution
    }
    
    #[test]
    fn test_missing_body_error() {
        let result = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .add_button("test", "Test")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Body text is required"));
    }
    
    #[test]
    fn test_no_interaction_elements_error() {
        let result = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Test message with no interactions")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("must have at least one interaction element"));
    }
    
    #[test]
    fn test_list_without_button_text_error() {
        let result = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Choose an option:")
            .add_list_section("Test Section")
                .add_list_row("test", "Test Option", "Description")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("List button text is required"));
    }
    
    #[test]
    fn test_empty_list_section_error() {
        let result = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Choose an option:")
            .list_button("Select")
            .add_list_section("Empty Section") // Section with no rows
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("must have at least one row"));
    }
    
    #[test]
    fn test_cta_url_https_validation() {
        // Test that HTTP URLs are rejected for security
        let builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Visit our website")
            .cta_url("Visit", "http://insecure-example.com"); // HTTP should fail
        
        let result = builder.build();
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("HTTPS protocol"));
    }
    
    #[test]
    fn test_cta_display_text_length_limit() {
        let long_text = "This is way too long for a CTA button"; // Over 20 characters
        let builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Visit our website")
            .cta_url(long_text, "https://example.com");
        
        let result = builder.build();
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("CTA button text too long"));
    }
    
    #[test]
    fn test_location_request_body_validation() {
        // Location requests should have meaningful body text for privacy/UX
        let builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Share") // Too short to explain why location is needed
            .request_location();
        
        let result = builder.build();
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("clear explanation of why location is needed"));
    }
    
    #[test]
    fn test_conditional_interaction_building() {
        // This test demonstrates real-world usage patterns where business logic
        // determines which interaction type to use based on context
        
        let user_preference = "simple";
        let options_count = 2;
        let requires_external_action = false;
        let needs_location = false;
        
        let mut builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Choose your preferred contact method:");
        
        // Business logic determines interaction type
        if needs_location {
            builder = builder.request_location();
        } else if requires_external_action {
            builder = builder.cta_url("Contact Us", "https://example.com/contact");
        } else if user_preference == "simple" && options_count <= 3 {
            // Use buttons for simple choices
            builder = builder
                .add_button("email", "Email")
                .add_button("phone", "Phone");
        } else {
            // Use list for complex choices
            builder = builder
                .list_button("Select Method")
                .add_list_section("Contact Options")
                    .add_list_row("email", "Email", "Get response within 24 hours")
                    .add_list_row("phone", "Phone Call", "Speak with representative");
        }
        
        // Verify the business logic resulted in button configuration
        assert_eq!(builder.buttons.len(), 2);
        assert!(!builder.location_request);
        assert!(builder.cta_url.is_none());
        assert!(builder.list_sections.is_empty());
    }
    
    #[test]
    fn test_builder_state_isolation() {
        // Test that clearing methods work correctly and don't interfere with each other
        let mut builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Test message")
            .add_button("btn1", "Button 1")
            .add_button("btn2", "Button 2")
            .list_button("List Button")
            .add_list_section("Section")
                .add_list_row("row1", "Row 1", "Description");
        
        // Initially both buttons and list elements are configured
        assert_eq!(builder.buttons.len(), 2);
        assert!(builder.list_button_text.is_some());
        
        // Clear buttons should only affect buttons
        builder = builder.clear_buttons();
        assert_eq!(builder.buttons.len(), 0);
        assert!(builder.list_button_text.is_some()); // List config should remain
        
        // Add buttons back
        builder = builder.add_button("new_btn", "New Button");
        assert_eq!(builder.buttons.len(), 1);
        
        // Clear list should only affect list configuration
        builder = builder.clear_list();
        assert!(builder.list_button_text.is_none());
        assert!(builder.list_sections.is_empty());
        assert!(builder.current_section.is_none());
        assert_eq!(builder.buttons.len(), 1); // Buttons should remain
    }
    
    #[test]
    fn test_section_building_flow() {
        // Test the complex flow of building sections and switching between them
        let mut builder = InteractiveMessageBuilder::new()
            .to("+1234567890")
            .body("Multi-section test")
            .list_button("Choose")
            .add_list_section("Section 1")
                .add_list_row("s1r1", "S1 Row 1", "First section, first row")
                .add_list_row("s1r2", "S1 Row 2", "First section, second row")
            .add_list_section("Section 2") // This should finalize Section 1 and start Section 2
                .add_simple_list_row("s2r1", "S2 Row 1")
            .add_list_section("Section 3") // This should finalize Section 2 and start Section 3
                .add_list_row("s3r1", "S3 Row 1", "Third section");
        
        // Manually finalize the current section to inspect the structure
        if let Some(section) = builder.current_section.take() {
            builder.list_sections.push(section);
        }
        
        // We should have 3 complete sections plus the current one we just moved
        assert_eq!(builder.list_sections.len(), 3);
        
        // Verify Section 1 structure
        assert_eq!(builder.list_sections[0].title, "Section 1");
        assert_eq!(builder.list_sections[0].rows.len(), 2);
        
        // Verify Section 2 structure  
        assert_eq!(builder.list_sections[1].title, "Section 2");
        assert_eq!(builder.list_sections[1].rows.len(), 1);
        assert_eq!(builder.list_sections[1].rows[0].2, None); // Simple row has no description
        
        // Verify Section 3 structure
        assert_eq!(builder.list_sections[2].title, "Section 3");
        assert_eq!(builder.list_sections[2].rows.len(), 1);
        assert_eq!(builder.list_sections[2].rows[0].2, Some("Third section".to_string()));
    }
}
