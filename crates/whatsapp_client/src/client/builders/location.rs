use crate::{
    errors::WhatsAppResult,
    client::message_types::LocationMessage,
};

/// Builder for creating location messages with fluent interface
/// 
/// This builder represents a fundamental paradigm shift from media-based
/// messages to coordinate-based communication. Location messages enable
/// rich geographic communication for business scenarios like delivery
/// tracking, meeting coordination, and location-based services.
/// 
/// Unlike media builders, this builder works with geographic coordinates
/// and optional descriptive metadata to create meaningful location sharing.
/// 
/// # Core Concepts
/// - **Coordinates**: Latitude (-90 to 90) and longitude (-180 to 180)
/// - **Name**: Human-readable location identifier ("Central Park")
/// - **Address**: Full address or additional context
/// - **Validation**: Ensures coordinates are within valid Earth bounds
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::LocationMessageBuilder;
/// // Business location sharing
/// let message = LocationMessageBuilder::new()
///     .to("+1234567890")
///     .coordinates(40.7580, -73.9855)
///     .name("Our NYC Office")
///     .address("123 Broadway, Manhattan, NY 10036")
///     .build()?;
/// 
/// // Simple coordinate sharing
/// let message = LocationMessageBuilder::new()
///     .to("+1234567890")
///     .latitude(40.7580)
///     .longitude(-73.9855)
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct LocationMessageBuilder {
    to: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    name: Option<String>,
    address: Option<String>,
}

impl LocationMessageBuilder {
    /// Create a new location message builder
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
    
    /// Set both latitude and longitude coordinates at once
    /// 
    /// This is the most common way to set coordinates when you have
    /// both values available. It's more ergonomic than setting them
    /// separately and reduces the chance of forgetting one coordinate.
    /// 
    /// # Arguments
    /// * `lat` - Latitude in decimal degrees (-90.0 to 90.0)
    /// * `lng` - Longitude in decimal degrees (-180.0 to 180.0)
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::LocationMessageBuilder;
    /// // Times Square coordinates
    /// let message = LocationMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .coordinates(40.7580, -73.9855)
    ///     .name("Times Square")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn coordinates(mut self, lat: f64, lng: f64) -> Self {
        self.latitude = Some(lat);
        self.longitude = Some(lng);
        self
    }
    
    /// Set the latitude coordinate individually
    /// 
    /// Use this when you need to set coordinates separately,
    /// perhaps from different data sources or in different
    /// parts of your application logic.
    /// 
    /// # Arguments
    /// * `lat` - Latitude in decimal degrees (-90.0 to 90.0)
    /// 
    /// # Note
    /// Both latitude and longitude must be set before building.
    /// Consider using `coordinates()` if you have both values available.
    pub fn latitude(mut self, lat: f64) -> Self {
        self.latitude = Some(lat);
        self
    }
    
    /// Set the longitude coordinate individually
    /// 
    /// Use this when you need to set coordinates separately,
    /// perhaps from different data sources or in different
    /// parts of your application logic.
    /// 
    /// # Arguments
    /// * `lng` - Longitude in decimal degrees (-180.0 to 180.0)
    /// 
    /// # Note
    /// Both latitude and longitude must be set before building.
    /// Consider using `coordinates()` if you have both values available.
    pub fn longitude(mut self, lng: f64) -> Self {
        self.longitude = Some(lng);
        self
    }
    
    /// Set a human-readable name for the location
    /// 
    /// Names help recipients understand what the location represents
    /// before they open it in their map application. This is especially
    /// valuable for business locations, landmarks, or meeting points.
    /// 
    /// # Arguments
    /// * `location_name` - Descriptive name for the location
    /// 
    /// # Best Practices
    /// - Use recognizable names ("Central Park", "Starbucks on Main St")
    /// - Keep names concise but descriptive
    /// - Include business names for commercial locations
    /// - Avoid coordinates or technical identifiers in the name
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::LocationMessageBuilder;
    /// let message = LocationMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .coordinates(40.7580, -73.9855)
    ///     .name("Conference Center - Main Entrance")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn name(mut self, location_name: &str) -> Self {
        self.name = Some(location_name.to_string());
        self
    }
    
    /// Set a detailed address or description for the location
    /// 
    /// Addresses provide additional context and help with navigation.
    /// They're especially useful for precise locations within larger
    /// areas (like specific building entrances or parking areas).
    /// 
    /// # Arguments
    /// * `location_address` - Full address or additional description
    /// 
    /// # Usage Patterns
    /// - **Full addresses**: "123 Main Street, Suite 456, New York, NY 10001"
    /// - **Navigation hints**: "Use the north entrance, visitor parking available"
    /// - **Context clues**: "Behind the shopping center, near the food court"
    /// - **Meeting instructions**: "Second floor conference room, ask for John"
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::LocationMessageBuilder;
    /// let message = LocationMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .coordinates(40.7580, -73.9855)
    ///     .name("Client Office")
    ///     .address("456 Business Plaza, 15th Floor - Use visitor entrance on West side")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn address(mut self, location_address: &str) -> Self {
        self.address = Some(location_address.to_string());
        self
    }
    
    /// Remove any previously set location name
    /// 
    /// Useful for conditional logic where you might want to
    /// clear a name based on business rules or user preferences.
    pub fn without_name(mut self) -> Self {
        self.name = None;
        self
    }
    
    /// Remove any previously set address
    /// 
    /// Useful for conditional logic where you might want to
    /// provide only basic location data without detailed addressing.
    pub fn without_address(mut self) -> Self {
        self.address = None;
        self
    }
    
    /// Build the location message
    /// 
    /// This validates all configuration and creates the final LocationMessage.
    /// Location messages have unique validation requirements focused on
    /// geographic coordinate bounds and logical consistency.
    /// 
    /// # Validation Process
    /// 1. Recipient phone number must be valid E.164 format
    /// 2. Both latitude AND longitude must be provided
    /// 3. Latitude must be between -90.0 and 90.0 (North/South poles)
    /// 4. Longitude must be between -180.0 and 180.0 (International Date Line)
    /// 5. Coordinates must represent valid Earth locations
    /// 
    /// # Geographic Context
    /// The validation ensures coordinates represent real Earth locations:
    /// - Latitude 0° = Equator, +90° = North Pole, -90° = South Pole
    /// - Longitude 0° = Prime Meridian, +180°/-180° = International Date Line
    /// 
    /// # Error Scenarios
    /// - Missing recipient or coordinates
    /// - Invalid phone number format
    /// - Coordinates outside valid Earth bounds
    /// - Missing either latitude or longitude (both required)
    pub fn build(self) -> WhatsAppResult<LocationMessage> {
        let to = self.to.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required for location messages".to_string()
            )
        })?;
        
        let latitude = self.latitude.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Latitude coordinate is required. Use coordinates(lat, lng) or latitude(lat) method.".to_string()
            )
        })?;
        
        let longitude = self.longitude.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Longitude coordinate is required. Use coordinates(lat, lng) or longitude(lng) method.".to_string()
            )
        })?;
        
        // Create the base location message with validated coordinates
        let mut message = LocationMessage::new(&to, latitude, longitude)?;
        
        // Add optional descriptive information
        if let Some(location_name) = self.name {
            message = message.with_location_name(&location_name);
        }
        
        if let Some(location_address) = self.address {
            message = message.with_location_address(&location_address);
        }
        
        Ok(message)
    }
    
    /// Validate coordinates before building (utility method)
    /// 
    /// This helper method allows you to validate coordinates
    /// independently of the full message building process.
    /// Useful for geographic calculations or pre-validation.
    /// 
    /// # Arguments
    /// * `lat` - Latitude to validate (-90.0 to 90.0)
    /// * `lng` - Longitude to validate (-180.0 to 180.0)
    /// 
    /// # Returns
    /// Ok(()) if coordinates are valid, detailed error otherwise
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::LocationMessageBuilder;
    /// // Validate user input before processing
    /// match LocationMessageBuilder::validate_coordinates(40.7580, -73.9855) {
    ///     Ok(_) => println!("Coordinates are valid Earth locations"),
    ///     Err(e) => println!("Invalid coordinates: {}", e),
    /// }
    /// ```
    pub fn validate_coordinates(lat: f64, lng: f64) -> WhatsAppResult<()> {
        LocationMessage::validate_location_coordinates(lat, lng)
    }
    
    /// Calculate distance between two coordinate pairs
    /// 
    /// This utility method helps with geographic calculations
    /// that might be useful in location-based business logic.
    /// Uses the Haversine formula for great-circle distance.
    /// 
    /// # Arguments
    /// * `lat1`, `lng1` - First location coordinates
    /// * `lat2`, `lng2` - Second location coordinates
    /// 
    /// # Returns
    /// Distance in kilometers between the two points
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::LocationMessageBuilder;
    /// // Calculate distance between two business locations
    /// let distance = LocationMessageBuilder::calculate_distance(
    ///     40.7580, -73.9855, // Times Square
    ///     40.7484, -73.9857  // Empire State Building
    /// );
    /// println!("Distance: {:.1} km", distance);
    /// ```
    pub fn calculate_distance(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
        // Create a temporary location message to use its distance calculation
        // This reuses the proven Haversine implementation
        if let Ok(temp_location) = LocationMessage::new("+10000000000", lat1, lng1) {
            temp_location.distance_to(lat2, lng2)
        } else {
            0.0 // Return 0 for invalid coordinates
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_location_message_with_coordinates_method() {
        let message = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(40.7580, -73.9855)
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.coordinates(), (40.7580, -73.9855));
        assert_eq!(message.location_name(), None);
        assert_eq!(message.location_address(), None);
    }
    
    #[test]
    fn test_location_message_with_separate_coordinates() {
        let message = LocationMessageBuilder::new()
            .to("+1234567890")
            .latitude(40.7580)
            .longitude(-73.9855)
            .build()
            .unwrap();
        
        assert_eq!(message.coordinates(), (40.7580, -73.9855));
    }
    
    #[test]
    fn test_location_message_with_full_details() {
        let message = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(40.7580, -73.9855)
            .name("Times Square")
            .address("Manhattan, New York, NY 10036, USA")
            .build()
            .unwrap();
        
        assert_eq!(message.location_name(), Some("Times Square"));
        assert_eq!(message.location_address(), Some("Manhattan, New York, NY 10036, USA"));
        assert!(message.has_description());
    }
    
    #[test]
    fn test_coordinate_override() {
        // Later coordinate setting should override earlier ones
        let message = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(0.0, 0.0)
            .coordinates(40.7580, -73.9855) // This should override
            .build()
            .unwrap();
        
        assert_eq!(message.coordinates(), (40.7580, -73.9855));
    }
    
    #[test]
    fn test_individual_coordinate_override() {
        let message = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(0.0, 0.0)
            .latitude(40.7580) // Override just latitude
            .build()
            .unwrap();
        
        assert_eq!(message.coordinates(), (40.7580, 0.0));
    }
    
    #[test]
    fn test_metadata_removal() {
        let message = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(40.7580, -73.9855)
            .name("Test Location")
            .address("Test Address")
            .without_name()
            .without_address()
            .build()
            .unwrap();
        
        assert_eq!(message.location_name(), None);
        assert_eq!(message.location_address(), None);
        assert!(!message.has_description());
    }
    
    #[test]
    fn test_missing_recipient_error() {
        let result = LocationMessageBuilder::new()
            .coordinates(40.7580, -73.9855)
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Recipient phone number is required"));
    }
    
    #[test]
    fn test_missing_latitude_error() {
        let result = LocationMessageBuilder::new()
            .to("+1234567890")
            .longitude(-73.9855)
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Latitude coordinate is required"));
    }
    
    #[test]
    fn test_missing_longitude_error() {
        let result = LocationMessageBuilder::new()
            .to("+1234567890")
            .latitude(40.7580)
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Longitude coordinate is required"));
    }
    
    #[test]
    fn test_invalid_coordinates() {
        let result = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(91.0, 0.0) // Invalid latitude
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_coordinate_validation_utility() {
        // Valid coordinates
        assert!(LocationMessageBuilder::validate_coordinates(40.7580, -73.9855).is_ok());
        assert!(LocationMessageBuilder::validate_coordinates(90.0, 180.0).is_ok());
        
        // Invalid coordinates
        assert!(LocationMessageBuilder::validate_coordinates(91.0, 0.0).is_err());
        assert!(LocationMessageBuilder::validate_coordinates(0.0, 181.0).is_err());
    }
    
    #[test]
    fn test_distance_calculation_utility() {
        // Distance between Times Square and Empire State Building
        let distance = LocationMessageBuilder::calculate_distance(
            40.7580, -73.9855, // Times Square
            40.7484, -73.9857  // Empire State Building
        );
        
        // Should be approximately 1.06 km
        assert!((distance - 1.06).abs() < 0.1);
    }
    
    #[test]
    fn test_fluent_interface_geographic_workflow() {
        // Simulate a real-world geographic workflow
        let business_lat = 40.7580;
        let business_lng = -73.9855;
        let business_name = "NYC Office";
        let provide_directions = true;
        
        let mut builder = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(business_lat, business_lng)
            .name(business_name);
        
        if provide_directions {
            builder = builder.address("Use the main entrance on Broadway - visitor parking available");
        }
        
        let message = builder.build().unwrap();
        assert_eq!(message.location_name(), Some("NYC Office"));
        assert!(message.has_description());
    }
    
    #[test]
    fn test_conditional_metadata_for_location_types() {
        // Simulate different location sharing scenarios
        let location_type = "business"; // Could be "home", "business", "meeting"
        let include_address = location_type == "business";
        
        let mut builder = LocationMessageBuilder::new()
            .to("+1234567890")
            .coordinates(40.7580, -73.9855);
        
        match location_type {
            "business" => {
                builder = builder.name("Our Office");
                if include_address {
                    builder = builder.address("123 Business Street, Suite 456");
                }
            },
            "meeting" => {
                builder = builder.name("Meeting Point");
            },
            "home" => {
                // Might not include detailed address for privacy
                builder = builder.name("My Location");
            },
            _ => {}
        }
        
        let message = builder.build().unwrap();
        assert_eq!(message.location_name(), Some("Our Office"));
        assert_eq!(message.location_address(), Some("123 Business Street, Suite 456"));
    }
}
