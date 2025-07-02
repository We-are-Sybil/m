# Event-Driven WhatsApp Bot Integration Guide

A comprehensive guide for building services that integrate with our event-driven WhatsApp bot infrastructure using Apache Kafka.

## Table of Contents

- [System Overview](#system-overview)
- [Quick Start](#quick-start)
- [Event System Architecture](#event-system-architecture)
- [Event Types and Formats](#event-types-and-formats)
- [Rust Integration](#rust-integration)
- [Python Integration](#python-integration)
- [Node.js Integration](#nodejs-integration)
- [Testing and Debugging](#testing-and-debugging)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## System Overview

Our WhatsApp bot uses an event-driven architecture where messages flow through Kafka topics, enabling multiple services to process and respond to user interactions independently.

### Architecture Flow

```
WhatsApp → Webhook Service → Kafka Topics → Your Services → Response Handler → WhatsApp
```

**Core Components:**
- **Webhook Service**: Receives WhatsApp webhooks and publishes events
- **Kafka Cluster**: Message broker with topics for different event types
- **Your Services**: AI processors, business logic, analytics, etc.
- **Response Handler**: Converts responses back to WhatsApp messages

### Key Benefits

- **Scalability**: Add new services without affecting existing ones
- **Reliability**: Automatic retries and dead letter queues
- **Flexibility**: Services can be written in any language
- **Observability**: All events flow through observable Kafka topics

## Quick Start

### 1. Start the Infrastructure

```bash
# Clone the repository
git clone <repository-url>
cd m

# Start Kafka cluster and webhook service
podman-compose up -d

# Verify services are healthy
podman-compose ps
```

### 2. Verify Event Flow

```bash
# Run the test consumer to see events flowing
cargo run --bin test_consumer

# In another terminal, send a test message
curl -X POST http://localhost:8000/webhook \
  -H "Content-Type: application/json" \
  -d '{
    "object": "whatsapp_business_account",
    "entry": [{
      "id": "test",
      "changes": [{
        "value": {
          "messaging_product": "whatsapp",
          "messages": [{
            "from": "1234567890",
            "id": "test123",
            "timestamp": "1640995200",
            "text": {"body": "Hello!"},
            "type": "text"
          }]
        },
        "field": "messages"
      }]
    }]
  }'
```

You should see the event appear in your test consumer, confirming the system is working.

## Event System Architecture

### Kafka Topics

Our system uses the following topic structure:

| Topic | Purpose | Event Types |
|-------|---------|-------------|
| `conversation.messages` | Incoming user messages | Text, Image, Audio, Video, Document, Location, Contact |
| `conversation.interactions` | User interactions with buttons/lists | Button clicks, List selections |
| `conversation.responses` | Outgoing responses to users | Text, Interactive, Media responses |
| `conversation.failures` | Failed message processing | Processing errors, validation failures |
| `*.retry` | Retry queues | Failed events for reprocessing |
| `*.dlq` | Dead letter queues | Events that failed all retries |

### Event Envelope Structure

All events are wrapped in a standard envelope:

```json
{
  "event_id": "uuid-v4",
  "timestamp": "2025-01-02T12:00:00Z",
  "event_type": "MessageReceived",
  "version": "1.0",
  "data": { /* Event-specific data */ },
  "metadata": {
    "processed_by": "service-name",
    "processing_timestamp": "2025-01-02T12:00:01Z"
  },
  "attempt_count": 1,
  "max_attempts": 3
}
```

### Partitioning Strategy

Events are partitioned by phone number to ensure:
- **Ordering**: Messages from the same user are processed in order
- **Scalability**: Load is distributed across partitions
- **Parallel Processing**: Different users can be processed simultaneously

## Event Types and Formats

### MessageReceived Events

Triggered when users send messages to the WhatsApp bot.

```json
{
  "message_id": "wamid.ABC123",
  "from_phone": "+1234567890",
  "message_type": "Text|Image|Audio|Video|Document|Location|Contact|Sticker",
  "content": {
    // Content varies by message type - see examples below
  },
  "received_at": "2025-01-02T12:00:00Z",
  "metadata": {
    "context_message_id": "wamid.XYZ789", // If replying to a message
    "processed_by": "webhook_event_publisher"
  }
}
```

#### Content Examples

**Text Message:**
```json
"content": {
  "Text": {
    "body": "Hello, I need help with my order"
  }
}
```

**Media Message (Image/Audio/Video/Document):**
```json
"content": {
  "Media": {
    "media_id": "media123",
    "caption": "Here's my receipt",
    "mime_type": "image/jpeg"
  }
}
```

**Location Message:**
```json
"content": {
  "Location": {
    "latitude": 37.7749,
    "longitude": -122.4194,
    "name": "San Francisco",
    "address": "San Francisco, CA"
  }
}
```

**Contact Message:**
```json
"content": {
  "Contact": {
    "name": "John Doe",
    "phone_number": "+1234567890",
    "email": "john@example.com"
  }
}
```

### InteractionReceived Events

Triggered when users interact with buttons or lists.

```json
{
  "original_message_id": "wamid.ABC123",
  "from_phone": "+1234567890",
  "interaction_type": "ButtonReply|ListReply",
  "selection": {
    // Selection varies by interaction type
  },
  "received_at": "2025-01-02T12:00:00Z"
}
```

#### Selection Examples

**Button Selection:**
```json
"selection": {
  "Button": {
    "id": "help_button",
    "title": "Get Help"
  }
}
```

**List Selection:**
```json
"selection": {
  "List": {
    "id": "billing_option",
    "title": "Billing Question",
    "description": "Questions about your bill"
  }
}
```

### ResponseReady Events

Published by your services to send responses back to users.

```json
{
  "original_message_id": "wamid.ABC123",
  "to_phone": "+1234567890",
  "response_type": "Text|Interactive|Media|Template",
  "content": {
    // Content varies by response type
  },
  "generated_at": "2025-01-02T12:00:00Z",
  "priority": "Low|Normal|Urgent"
}
```

#### Response Content Examples

**Text Response:**
```json
"content": {
  "Text": {
    "message": "Thanks for your message! How can I help you today?"
  }
}
```

**Interactive Response (Buttons):**
```json
"content": {
  "Interactive": {
    "body_text": "How can I help you?",
    "buttons": [
      {"id": "support", "title": "Support"},
      {"id": "billing", "title": "Billing"}
    ]
  }
}
```

**Interactive Response (List):**
```json
"content": {
  "List": {
    "body_text": "Choose a topic:",
    "button_text": "Select Option",
    "sections": [{
      "title": "Common Questions",
      "rows": [
        {"id": "faq1", "title": "Account Help", "description": "Account-related questions"},
        {"id": "faq2", "title": "Technical Support", "description": "Technical issues"}
      ]
    }]
  }
}
```

### MessageFailed Events

Published when message processing fails after all retries.

```json
{
  "message_id": "wamid.ABC123",
  "phone": "+1234567890",
  "failure_type": "SerializationError|ProcessingTimeout|ExternalServiceError|ValidationError|UnknownError",
  "error_details": "Detailed error description",
  "attempt_count": 3,
  "failed_at": "2025-01-02T12:00:00Z"
}
```

## Rust Integration

For Rust services, use our `common` crate which provides type-safe event handling and automatic Kafka integration.

### Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
common = { path = "../common" }
tokio = { version = "1", features = ["full"] }
tracing = "0.1"
```

### Environment Configuration

Create a `.env` file:

```bash
KAFKA_BOOTSTRAP_SERVERS=localhost:9092,localhost:9094,localhost:9096
KAFKA_CONSUMER_GROUP_ID=my-ai-service
KAFKA_TIMEOUT_MS=5000
KAFKA_SECURITY_PROTOCOL=PLAINTEXT
```

### Consumer Example

```rust
use common::{
    KafkaEventBus, KafkaConfig, EventBus, MessageReceived, ResponseReady,
    SubscriptionConfig, ProcessingResult, EventEnvelope, ResponseType, ResponseContent
};
use std::sync::Arc;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    
    // Initialize Kafka event bus
    let kafka_config = KafkaConfig::from_env()?;
    let event_bus = Arc::new(KafkaEventBus::new(kafka_config).await?);
    
    // Subscribe to incoming messages
    let config = SubscriptionConfig {
        consumer_group: "ai-processor".to_string(),
        ..Default::default()
    };
    
    let bus_clone = event_bus.clone();
    event_bus.subscribe::<MessageReceived, _>(
        config,
        move |envelope: EventEnvelope<MessageReceived>| {
            let bus = bus_clone.clone();
            tokio::spawn(async move {
                process_message(bus, envelope).await
            });
            Ok(ProcessingResult::Success)
        }
    ).await?;
    
    info!("AI processor started, waiting for messages...");
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn process_message(
    event_bus: Arc<KafkaEventBus>,
    envelope: EventEnvelope<MessageReceived>
) -> Result<(), Box<dyn std::error::Error>> {
    let message = &envelope.data;
    
    // Process the message with your AI/business logic
    let response_text = match &message.content {
        common::MessageContent::Text { body } => {
            // AI processing logic here
            format!("I understand you said: '{}'. How can I help?", body)
        }
        _ => "I received your message. How can I help you?".to_string()
    };
    
    // Create response event
    let response = ResponseReady {
        original_message_id: message.message_id.clone(),
        to_phone: message.from_phone.clone(),
        response_type: ResponseType::Text,
        content: ResponseContent::Text {
            message: response_text
        },
        generated_at: chrono::Utc::now(),
        priority: common::ResponsePriority::Normal,
    };
    
    // Publish response
    event_bus.publish(response).await?;
    info!("Published response for message {}", message.message_id);
    
    Ok(())
}
```

### Producer Example

```rust
use common::{KafkaEventBus, KafkaConfig, EventBus, ResponseReady, ResponseType, ResponseContent};
use std::sync::Arc;

async fn send_response(
    event_bus: Arc<KafkaEventBus>,
    to_phone: String,
    message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = ResponseReady {
        original_message_id: "generated".to_string(),
        to_phone,
        response_type: ResponseType::Text,
        content: ResponseContent::Text { message },
        generated_at: chrono::Utc::now(),
        priority: common::ResponsePriority::Normal,
    };
    
    event_bus.publish(response).await?;
    Ok(())
}
```

## Python Integration

For Python services, use the `confluent-kafka` library with our event schemas.

### Setup

```bash
# Install required packages
pip install confluent-kafka pydantic python-dotenv

# Or with uv (recommended)
uv add confluent-kafka pydantic python-dotenv
```

### Event Models

Create `events.py`:

```python
from pydantic import BaseModel
from typing import Optional, Dict, Any, Union
from datetime import datetime
from enum import Enum

class MessageType(str, Enum):
    TEXT = "Text"
    IMAGE = "Image"
    AUDIO = "Audio"
    VIDEO = "Video"
    DOCUMENT = "Document"
    LOCATION = "Location"
    CONTACT = "Contact"
    STICKER = "Sticker"

class TextContent(BaseModel):
    body: str

class MediaContent(BaseModel):
    media_id: str
    caption: Optional[str] = None
    mime_type: str

class LocationContent(BaseModel):
    latitude: float
    longitude: float
    name: Optional[str] = None
    address: Optional[str] = None

class ContactContent(BaseModel):
    name: str
    phone_number: str
    email: Optional[str] = None

class MessageReceived(BaseModel):
    message_id: str
    from_phone: str
    message_type: MessageType
    content: Dict[str, Any]  # Union of content types
    received_at: datetime
    metadata: Dict[str, str]

class EventEnvelope(BaseModel):
    event_id: str
    timestamp: datetime
    event_type: str
    version: str
    data: Dict[str, Any]  # Will contain MessageReceived data
    metadata: Dict[str, str]
    attempt_count: int
    max_attempts: int

class ResponseReady(BaseModel):
    original_message_id: str
    to_phone: str
    response_type: str
    content: Dict[str, Any]
    generated_at: datetime
    priority: str = "Normal"
```

### Consumer Example

Create `consumer.py`:

```python
import json
import os
from confluent_kafka import Consumer, KafkaError
from events import EventEnvelope, MessageReceived
from dotenv import load_dotenv
import logging

# Load environment variables
load_dotenv()

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class MessageConsumer:
    def __init__(self):
        self.consumer = Consumer({
            'bootstrap.servers': os.getenv('KAFKA_BOOTSTRAP_SERVERS', 'localhost:9092'),
            'group.id': os.getenv('KAFKA_CONSUMER_GROUP_ID', 'python-ai-service'),
            'auto.offset.reset': 'earliest',
            'enable.auto.commit': False,
        })
        
        self.consumer.subscribe(['conversation.messages'])
    
    def process_message(self, message_data: MessageReceived) -> str:
        """Process the message with your AI/business logic"""
        content = message_data.content
        
        if message_data.message_type == "Text" and "Text" in content:
            user_text = content["Text"]["body"]
            # Add your AI processing logic here
            return f"I understand you said: '{user_text}'. How can I help?"
        else:
            return "I received your message. How can I help you?"
    
    def run(self):
        logger.info("Starting Python message consumer...")
        
        try:
            while True:
                msg = self.consumer.poll(1.0)
                
                if msg is None:
                    continue
                    
                if msg.error():
                    if msg.error().code() == KafkaError._PARTITION_EOF:
                        continue
                    else:
                        logger.error(f"Consumer error: {msg.error()}")
                        continue
                
                try:
                    # Parse the event envelope
                    envelope_data = json.loads(msg.value().decode('utf-8'))
                    envelope = EventEnvelope(**envelope_data)
                    
                    # Extract the message data
                    message_data = MessageReceived(**envelope.data)
                    
                    logger.info(f"Processing message {message_data.message_id} from {message_data.from_phone}")
                    
                    # Process the message
                    response_text = self.process_message(message_data)
                    
                    # Here you would publish a response event
                    # See producer example below
                    
                    # Commit the offset
                    self.consumer.commit(msg)
                    
                    logger.info(f"Successfully processed message {message_data.message_id}")
                    
                except Exception as e:
                    logger.error(f"Error processing message: {e}")
                    # Depending on error type, you might want to commit or not
                    self.consumer.commit(msg)  # Skip bad messages
                    
        except KeyboardInterrupt:
            logger.info("Shutting down consumer...")
        finally:
            self.consumer.close()

if __name__ == "__main__":
    consumer = MessageConsumer()
    consumer.run()
```

### Producer Example

Create `producer.py`:

```python
import json
import os
import uuid
from datetime import datetime
from confluent_kafka import Producer
from events import ResponseReady
from dotenv import load_dotenv
import logging

load_dotenv()

class ResponseProducer:
    def __init__(self):
        self.producer = Producer({
            'bootstrap.servers': os.getenv('KAFKA_BOOTSTRAP_SERVERS', 'localhost:9092'),
            'acks': 'all',
            'retries': 3,
            'enable.idempotence': True
        })
    
    def send_text_response(self, to_phone: str, message: str, original_message_id: str = "generated"):
        """Send a text response to a user"""
        
        # Create response event
        response = ResponseReady(
            original_message_id=original_message_id,
            to_phone=to_phone,
            response_type="Text",
            content={
                "Text": {
                    "message": message
                }
            },
            generated_at=datetime.utcnow(),
            priority="Normal"
        )
        
        # Create event envelope
        envelope = {
            "event_id": str(uuid.uuid4()),
            "timestamp": datetime.utcnow().isoformat(),
            "event_type": "ResponseReady",
            "version": "1.0",
            "data": response.dict(),
            "metadata": {
                "processed_by": "python-ai-service",
                "processing_timestamp": datetime.utcnow().isoformat()
            },
            "attempt_count": 0,
            "max_attempts": 3
        }
        
        # Publish to Kafka
        self.producer.produce(
            topic='conversation.responses',
            key=to_phone,
            value=json.dumps(envelope),
            callback=self._delivery_callback
        )
        
        # Wait for message to be delivered
        self.producer.flush()
    
    def _delivery_callback(self, err, msg):
        if err:
            logging.error(f"Message delivery failed: {err}")
        else:
            logging.info(f"Message delivered to {msg.topic()} [{msg.partition()}]")

# Usage example
if __name__ == "__main__":
    producer = ResponseProducer()
    producer.send_text_response(
        to_phone="+1234567890",
        message="Hello from Python service!",
        original_message_id="test123"
    )
```

### Environment Configuration

Create `.env`:

```bash
KAFKA_BOOTSTRAP_SERVERS=localhost:9092,localhost:9094,localhost:9096
KAFKA_CONSUMER_GROUP_ID=python-ai-service
```

## Node.js Integration

For Node.js services, use the `kafkajs` library.

### Setup

```bash
npm install kafkajs dotenv
# or
yarn add kafkajs dotenv
```

### Consumer Example

Create `consumer.js`:

```javascript
const { Kafka } = require('kafkajs');
require('dotenv').config();

class MessageConsumer {
    constructor() {
        this.kafka = Kafka({
            clientId: 'nodejs-ai-service',
            brokers: process.env.KAFKA_BOOTSTRAP_SERVERS?.split(',') || ['localhost:9092']
        });
        
        this.consumer = this.kafka.consumer({ 
            groupId: process.env.KAFKA_CONSUMER_GROUP_ID || 'nodejs-ai-service' 
        });
    }
    
    async processMessage(messageData) {
        // Process the message with your AI/business logic
        const content = messageData.content;
        
        if (messageData.message_type === 'Text' && content.Text) {
            const userText = content.Text.body;
            // Add your AI processing logic here
            return `I understand you said: '${userText}'. How can I help?`;
        } else {
            return 'I received your message. How can I help you?';
        }
    }
    
    async run() {
        await this.consumer.connect();
        await this.consumer.subscribe({ topic: 'conversation.messages' });
        
        console.log('Starting Node.js message consumer...');
        
        await this.consumer.run({
            eachMessage: async ({ topic, partition, message }) => {
                try {
                    // Parse the event envelope
                    const envelope = JSON.parse(message.value.toString());
                    const messageData = envelope.data;
                    
                    console.log(`Processing message ${messageData.message_id} from ${messageData.from_phone}`);
                    
                    // Process the message
                    const responseText = await this.processMessage(messageData);
                    
                    // Here you would publish a response event
                    // See producer example below
                    
                    console.log(`Successfully processed message ${messageData.message_id}`);
                    
                } catch (error) {
                    console.error('Error processing message:', error);
                }
            },
        });
    }
}

// Run the consumer
const consumer = new MessageConsumer();
consumer.run().catch(console.error);
```

### Producer Example

Create `producer.js`:

```javascript
const { Kafka } = require('kafkajs');
const { v4: uuidv4 } = require('uuid');
require('dotenv').config();

class ResponseProducer {
    constructor() {
        this.kafka = Kafka({
            clientId: 'nodejs-response-producer',
            brokers: process.env.KAFKA_BOOTSTRAP_SERVERS?.split(',') || ['localhost:9092']
        });
        
        this.producer = this.kafka.producer({
            idempotent: true,
            maxInFlightRequests: 1,
            retries: 3
        });
    }
    
    async connect() {
        await this.producer.connect();
    }
    
    async sendTextResponse(toPhone, message, originalMessageId = 'generated') {
        // Create response event
        const response = {
            original_message_id: originalMessageId,
            to_phone: toPhone,
            response_type: 'Text',
            content: {
                Text: {
                    message: message
                }
            },
            generated_at: new Date().toISOString(),
            priority: 'Normal'
        };
        
        // Create event envelope
        const envelope = {
            event_id: uuidv4(),
            timestamp: new Date().toISOString(),
            event_type: 'ResponseReady',
            version: '1.0',
            data: response,
            metadata: {
                processed_by: 'nodejs-ai-service',
                processing_timestamp: new Date().toISOString()
            },
            attempt_count: 0,
            max_attempts: 3
        };
        
        // Publish to Kafka
        await this.producer.send({
            topic: 'conversation.responses',
            messages: [{
                key: toPhone,
                value: JSON.stringify(envelope)
            }]
        });
        
        console.log(`Response sent to ${toPhone}`);
    }
    
    async disconnect() {
        await this.producer.disconnect();
    }
}

module.exports = { ResponseProducer };
```

## Testing and Debugging

### 1. Event Flow Testing

Use the built-in test consumer to verify events are flowing:

```bash
# Start the test consumer
cargo run --bin test_consumer

# Send test messages to see events
curl -X POST http://localhost:8000/webhook -H "Content-Type: application/json" -d @test_message.json
```

### 2. Topic Inspection

Check what's in your Kafka topics:

```bash
# List all topics
podman exec -it m-kafka-1 /opt/kafka/bin/kafka-topics.sh \
  --bootstrap-server localhost:9092 --list

# Read messages from a topic
podman exec -it m-kafka-1 /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic conversation.messages \
  --from-beginning
```

### 3. Consumer Group Monitoring

Monitor your consumer groups:

```bash
# List consumer groups
podman exec -it m-kafka-1 /opt/kafka/bin/kafka-consumer-groups.sh \
  --bootstrap-server localhost:9092 --list

# Check consumer group status
podman exec -it m-kafka-1 /opt/kafka/bin/kafka-consumer-groups.sh \
  --bootstrap-server localhost:9092 \
  --describe --group your-service-group
```

### 4. Dead Letter Queue Monitoring

Check for failed messages:

```bash
# Check dead letter queue
podman exec -it m-kafka-1 /opt/kafka/bin/kafka-console-consumer.sh \
  --bootstrap-server localhost:9092 \
  --topic conversation.messages.dlq \
  --from-beginning
```

## Best Practices

### Event Processing

1. **Idempotency**: Ensure your message processing is idempotent
2. **Error Handling**: Use appropriate error types for retries vs permanent failures
3. **Timeouts**: Set reasonable timeouts for external service calls
4. **Batching**: Process multiple events in batches when possible

### Consumer Configuration

```bash
# Recommended consumer settings
KAFKA_SESSION_TIMEOUT_MS=30000
KAFKA_HEARTBEAT_INTERVAL_MS=3000
KAFKA_MAX_POLL_INTERVAL_MS=300000
KAFKA_FETCH_MIN_BYTES=1024
KAFKA_FETCH_MAX_WAIT_MS=500
```

### Producer Configuration

```bash
# Recommended producer settings
KAFKA_ACKS=all
KAFKA_RETRIES=10
KAFKA_ENABLE_IDEMPOTENCE=true
KAFKA_COMPRESSION_TYPE=zstd
KAFKA_BATCH_SIZE=65536
KAFKA_LINGER_MS=5
```

### Service Design

1. **Single Responsibility**: Each service should handle one type of processing
2. **Stateless**: Services should be stateless for easy scaling
3. **Graceful Shutdown**: Handle shutdown signals properly
4. **Health Checks**: Implement health check endpoints
5. **Metrics**: Expose metrics for monitoring

### Schema Evolution

When updating event schemas:

1. **Backward Compatibility**: New fields should be optional
2. **Version Incrementing**: Increment version numbers for breaking changes
3. **Gradual Migration**: Deploy consumers before producers
4. **Testing**: Test with both old and new event formats

## Troubleshooting

### Common Issues

**Connection Refused**
```bash
# Check if Kafka is running
podman-compose ps

# Check network connectivity
podman exec -it webhook-service ping m-kafka-1
```

**Consumer Lag**
```bash
# Check consumer group lag
podman exec -it m-kafka-1 /opt/kafka/bin/kafka-consumer-groups.sh \
  --bootstrap-server localhost:9092 \
  --describe --group your-group
```

**Serialization Errors**
- Verify event schema matches your models
- Check JSON formatting
- Validate required fields are present

**Duplicate Processing**
- Ensure idempotent processing
- Check consumer group configuration
- Verify offset commit strategy

### Debug Logging

Enable debug logging for troubleshooting:

**Rust:**
```bash
RUST_LOG=debug cargo run
```

**Python:**
```python
logging.basicConfig(level=logging.DEBUG)
```

**Node.js:**
```javascript
// Set kafkajs log level
const kafka = Kafka({
    logLevel: logLevel.DEBUG
});
```

### Performance Monitoring

Monitor key metrics:
- **Consumer Lag**: How far behind consumers are
- **Throughput**: Messages processed per second
- **Error Rate**: Percentage of failed message processing
- **Response Time**: Time from message receipt to response

---

## Getting Help

For additional support:

1. Check the troubleshooting section above
2. Review Kafka logs: `podman-compose logs kafka-1`
3. Monitor consumer group status
4. Verify event schemas match your code
5. Test with the built-in test consumer first

This guide provides the foundation for building robust, scalable services that integrate with our event-driven WhatsApp bot infrastructure. Each service you build becomes part of a larger ecosystem that can handle millions of messages with high reliability and observability.
