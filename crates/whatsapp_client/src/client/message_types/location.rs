use crate::{
    errors::WhatsAppResult,
    client::validation::{validate_phone_number, validate_coordinates},
};
use serde::{Serialize, Deserialize};

/// A location message that can be sent via WhatsApp
/// 
/// Location messages display a map pin with optional name and address information.
/// They require latitude and longitude coordinates and can include descriptive text
/// to help users understand what location is being shared.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Location content configuration
    location: LocationContent,
}

/// Location message content structure
/// 
/// Contains the geographic coordinates and optional descriptive information
/// about the location being shared.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LocationContent {
    /// Latitude coordinate (-90 to 90)
    latitude: f64,
    /// Longitude coordinate (-180 to 180)
    longitude: f64,
    /// Optional name of the location (e.g., "Central Park")
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// Optional address or description (e.g., "123 Main St, New York, NY")
    #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
}

impl LocationMessage {
    /// Create a new location message with coordinates only
    /// 
    /// This creates a basic location message with just latitude and longitude.
    /// The location will appear as a map pin in WhatsApp without additional
    /// descriptive text.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `latitude` - Latitude coordinate (-90.0 to 90.0)
    /// * `longitude` - Longitude coordinate (-180.0 to 180.0)
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// // Share the coordinates for Times Square, NYC
    /// let message = LocationMessage::new("+1234567890", 40.7580, -73.9855)?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn new(to: &str, latitude: f64, longitude: f64) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_coordinates(latitude, longitude)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "location".to_string(),
            location: LocationContent {
                latitude,
                longitude,
                name: None,
                address: None,
            },
        })
    }
    
    /// Create a new location message with name
    /// 
    /// This creates a location message with coordinates and a descriptive name.
    /// The name helps users understand what the location represents.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `latitude` - Latitude coordinate (-90.0 to 90.0)
    /// * `longitude` - Longitude coordinate (-180.0 to 180.0)
    /// * `name` - Descriptive name for the location
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// let message = LocationMessage::with_name(
    ///     "+1234567890", 
    ///     40.7580, 
    ///     -73.9855, 
    ///     "Times Square"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_name(to: &str, latitude: f64, longitude: f64, name: &str) -> WhatsAppResult<Self> {
        let mut location = Self::new(to, latitude, longitude)?;
        location.location.name = Some(name.to_string());
        Ok(location)
    }
    
    /// Create a new location message with name and address
    /// 
    /// This creates a complete location message with coordinates, name, and address.
    /// This provides the most context for users receiving the location.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `latitude` - Latitude coordinate (-90.0 to 90.0)
    /// * `longitude` - Longitude coordinate (-180.0 to 180.0)
    /// * `name` - Descriptive name for the location
    /// * `address` - Full address or additional description
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// let message = LocationMessage::with_details(
    ///     "+1234567890", 
    ///     40.7580, 
    ///     -73.9855, 
    ///     "Times Square",
    ///     "Manhattan, New York, NY 10036, USA"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_details(
        to: &str, 
        latitude: f64, 
        longitude: f64, 
        name: &str, 
        address: &str
    ) -> WhatsAppResult<Self> {
        let mut location = Self::new(to, latitude, longitude)?;
        location.location.name = Some(name.to_string());
        location.location.address = Some(address.to_string());
        Ok(location)
    }
    
    /// Add a name to the location message
    /// 
    /// Sets a descriptive name for the location. This helps users understand
    /// what the shared location represents.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// let message = LocationMessage::new("+1234567890", 40.7580, -73.9855)?
    ///     .with_location_name("Our Office");
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_location_name(mut self, name: &str) -> Self {
        self.location.name = Some(name.to_string());
        self
    }
    
    /// Add an address to the location message
    /// 
    /// Sets an address or additional description for the location. This provides
    /// more context about where the location is situated.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// let message = LocationMessage::new("+1234567890", 40.7580, -73.9855)?
    ///     .with_location_address("123 Broadway, New York, NY");
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_location_address(mut self, address: &str) -> Self {
        self.location.address = Some(address.to_string());
        self
    }
    
    /// Get the recipient phone number
    pub fn recipient(&self) -> &str {
        &self.to
    }
    
    /// Get the latitude coordinate
    pub fn latitude(&self) -> f64 {
        self.location.latitude
    }
    
    /// Get the longitude coordinate
    pub fn longitude(&self) -> f64 {
        self.location.longitude
    }
    
    /// Get the coordinates as a tuple (latitude, longitude)
    pub fn coordinates(&self) -> (f64, f64) {
        (self.location.latitude, self.location.longitude)
    }
    
    /// Get the location name if set
    pub fn location_name(&self) -> Option<&str> {
        self.location.name.as_deref()
    }
    
    /// Get the location address if set
    pub fn location_address(&self) -> Option<&str> {
        self.location.address.as_deref()
    }
    
    /// Check if this location has descriptive information
    /// 
    /// Returns true if the location has either a name or address set.
    pub fn has_description(&self) -> bool {
        self.location.name.is_some() || self.location.address.is_some()
    }
    
    /// Validate coordinate values
    /// 
    /// This can be used to validate coordinates before creating a location message.
    /// Latitude must be between -90 and 90, longitude between -180 and 180.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// // Valid coordinates
    /// assert!(LocationMessage::validate_location_coordinates(40.7580, -73.9855).is_ok());
    /// 
    /// // Invalid coordinates
    /// assert!(LocationMessage::validate_location_coordinates(91.0, 0.0).is_err());
    /// ```
    pub fn validate_location_coordinates(latitude: f64, longitude: f64) -> WhatsAppResult<()> {
        validate_coordinates(latitude, longitude)
    }
    
    /// Calculate approximate distance to another location in kilometers
    /// 
    /// Uses the Haversine formula to calculate the great-circle distance
    /// between two points on Earth. This is useful for determining proximity
    /// or for display purposes.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::LocationMessage;
    /// let location = LocationMessage::new("+1234567890", 40.7580, -73.9855)?; // Times Square
    /// let distance = location.distance_to(40.7505, -73.9934); // Statue of Liberty
    /// // Distance is approximately 8.6 km
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn distance_to(&self, other_latitude: f64, other_longitude: f64) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;
        
        let lat1_rad = self.location.latitude.to_radians();
        let lat2_rad = other_latitude.to_radians();
        let delta_lat = (other_latitude - self.location.latitude).to_radians();
        let delta_lon = (other_longitude - self.location.longitude).to_radians();
        
        let a = (delta_lat / 2.0).sin().powi(2) +
               lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        
        EARTH_RADIUS_KM * c
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_location_message_basic() {
        let message = LocationMessage::new("+1234567890", 40.7580, -73.9855).unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.latitude(), 40.7580);
        assert_eq!(message.longitude(), -73.9855);
        assert_eq!(message.coordinates(), (40.7580, -73.9855));
        assert_eq!(message.location_name(), None);
        assert_eq!(message.location_address(), None);
        assert!(!message.has_description());
    }
    
    #[test]
    fn test_location_message_with_name() {
        let message = LocationMessage::with_name(
            "+1234567890", 
            40.7580, 
            -73.9855, 
            "Times Square"
        ).unwrap();
        
        assert_eq!(message.location_name(), Some("Times Square"));
        assert!(message.has_description());
    }
    
    #[test]
    fn test_location_message_with_details() {
        let message = LocationMessage::with_details(
            "+1234567890", 
            40.7580, 
            -73.9855, 
            "Times Square",
            "Manhattan, New York, NY 10036, USA"
        ).unwrap();
        
        assert_eq!(message.location_name(), Some("Times Square"));
        assert_eq!(message.location_address(), Some("Manhattan, New York, NY 10036, USA"));
        assert!(message.has_description());
    }
    
    #[test]
    fn test_location_message_builder_pattern() {
        let message = LocationMessage::new("+1234567890", 40.7580, -73.9855)
            .unwrap()
            .with_location_name("Our Office")
            .with_location_address("123 Broadway, New York, NY");
        
        assert_eq!(message.location_name(), Some("Our Office"));
        assert_eq!(message.location_address(), Some("123 Broadway, New York, NY"));
    }
    
    #[test]
    fn test_location_message_serialization_basic() {
        let message = LocationMessage::new("+1234567890", 40.7580, -73.9855).unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["recipient_type"], "individual");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "location");
        assert_eq!(json["location"]["latitude"], 40.7580);
        assert_eq!(json["location"]["longitude"], -73.9855);
        assert!(json["location"]["name"].is_null());
        assert!(json["location"]["address"].is_null());
    }
    
    #[test]
    fn test_location_message_serialization_with_details() {
        let message = LocationMessage::with_details(
            "+1234567890", 
            40.7580, 
            -73.9855, 
            "Times Square",
            "Manhattan, New York, NY"
        ).unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["location"]["name"], "Times Square");
        assert_eq!(json["location"]["address"], "Manhattan, New York, NY");
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = LocationMessage::new("invalid", 40.7580, -73.9855);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_coordinates() {
        // Invalid latitude (over 90)
        let result = LocationMessage::new("+1234567890", 91.0, 0.0);
        assert!(result.is_err());
        
        // Invalid longitude (over 180)
        let result = LocationMessage::new("+1234567890", 0.0, 181.0);
        assert!(result.is_err());
        
        // Invalid latitude (under -90)
        let result = LocationMessage::new("+1234567890", -91.0, 0.0);
        assert!(result.is_err());
        
        // Invalid longitude (under -180)
        let result = LocationMessage::new("+1234567890", 0.0, -181.0);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_coordinate_validation() {
        // Valid coordinates
        assert!(LocationMessage::validate_location_coordinates(40.7580, -73.9855).is_ok());
        assert!(LocationMessage::validate_location_coordinates(90.0, 180.0).is_ok());
        assert!(LocationMessage::validate_location_coordinates(-90.0, -180.0).is_ok());
        
        // Invalid coordinates
        assert!(LocationMessage::validate_location_coordinates(91.0, 0.0).is_err());
        assert!(LocationMessage::validate_location_coordinates(0.0, 181.0).is_err());
    }
    
    #[test]
    fn test_distance_calculation() {
        let times_square = LocationMessage::new("+1234567890", 40.7580, -73.9855).unwrap();
        
        // Distance to Empire State Building (approximately 0.7 km)
        let distance = times_square.distance_to(40.7484, -73.9857);
        assert!((distance - 1.06).abs() < 0.1); // Allow some tolerance for calculation differences
        
        // Distance to same location should be 0
        let same_distance = times_square.distance_to(40.7580, -73.9855);
        assert!(same_distance < 0.001); // Very close to 0
    }
    
    #[test]
    fn test_extreme_coordinates() {
        // Test with extreme but valid coordinates
        let north_pole = LocationMessage::new("+1234567890", 90.0, 0.0).unwrap();
        let south_pole = LocationMessage::new("+1234567890", -90.0, 0.0).unwrap();
        
        assert_eq!(north_pole.latitude(), 90.0);
        assert_eq!(south_pole.latitude(), -90.0);
        
        // Distance between poles should be approximately 20,015 km (half Earth's circumference)
        let pole_distance = north_pole.distance_to(-90.0, 0.0);
        assert!((pole_distance - 20015.0).abs() < 100.0); // Allow some tolerance
    }
}
