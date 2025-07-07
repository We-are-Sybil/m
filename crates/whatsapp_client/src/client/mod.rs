//! WhatsApp Business API Client Module
//!
//! Architecture:
//! - `core.rs`: Main client with HTTP handling and common functionality
//! - `message_types/`: Individual modules for each WhatsApp message type
//! - `builders/`: Builder patterns for constructing complex messages
//! - `responses.rs`: Response types and parsing
//! - `validation.rs`: Input validation utilities

pub mod core;
pub mod message_types;
pub mod builders;
pub mod responses;
pub mod validation;
