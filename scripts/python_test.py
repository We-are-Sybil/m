#!/usr/bin/env python3
"""
Test the Rust Kafka service by sending it webhook events and checking responses
"""

import json
import time
from datetime import datetime, timezone
from kafka import KafkaProducer, KafkaConsumer

def test_rust_service():
    # Your service configuration (from your .env)
    bootstrap_servers = ['localhost:9092', 'localhost:9094', 'localhost:9096']
    input_topic = 'whatsapp-webhook-input'
    output_topic = 'whatsapp-webhook-output'
    
    # Create test webhook event (matching your WebhookEvent struct)
    test_event = {
        "event_id": f"test-{int(time.time())}",
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "from_phone": "+1234567890",
        "message_type": "Text",
        "content": {
            "Text": {
                "body": "Help me with my order"
            }
        },
        "metadata": {}
    }
    
    print("🚀 Testing Rust Kafka Service")
    print(f"📤 Sending: {test_event}")
    
    # Send the test event
    producer = KafkaProducer(
        bootstrap_servers=bootstrap_servers,
        value_serializer=lambda v: json.dumps(v).encode('utf-8')
    )
    
    producer.send(input_topic, test_event)
    producer.flush()
    print("✅ Test event sent to input topic")
    
    # Listen for the AI response
    consumer = KafkaConsumer(
        output_topic,
        bootstrap_servers=bootstrap_servers,
        auto_offset_reset='latest',
        value_deserializer=lambda m: json.loads(m.decode('utf-8')),
        consumer_timeout_ms=10000  # 10 second timeout
    )
    
    print("⏳ Waiting for Rust service to process and respond...")
    
    for message in consumer:
        response = message.value
        print(f"🎉 Got response from Rust service:")
        print(f"   Original Event ID: {response.get('original_event_id')}")
        print(f"   Processed At: {response.get('processed_at')}")
        print(f"   Response: {response.get('response')}")
        
        # Check if it matches your expected format
        if response.get('original_event_id') == test_event['event_id']:
            print("✅ Response matches sent event!")
        else:
            print("❌ Response doesn't match sent event")
            
        break
    else:
        print("❌ No response received within timeout")
        print("   Check if your Rust service is running and processing messages")
    
    producer.close()
    consumer.close()

if __name__ == "__main__":
    test_rust_service()
