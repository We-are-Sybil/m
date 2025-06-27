#!/usr/bin/env python3
"""
Monitor what your Rust service is producing in real-time
"""

import json
from kafka import KafkaConsumer

def monitor_output():
    print("👀 Monitoring Rust service output...")
    print("   Send messages to test your service, press Ctrl+C to stop\n")
    
    consumer = KafkaConsumer(
        'whatsapp-webhook-output',
        bootstrap_servers=['localhost:9092', 'localhost:9094', 'localhost:9096'],
        auto_offset_reset='latest',  # Only new messages
        value_deserializer=lambda m: json.loads(m.decode('utf-8'))
    )
    
    try:
        for message in consumer:
            response = message.value
            print(f"📨 New response from Rust service:")
            print(f"   Event ID: {response.get('original_event_id')}")
            print(f"   Processed: {response.get('processed_at')}")
            print(f"   Response Type: {list(response.get('response', {}).keys())}")
            
            # Pretty print the actual response
            if 'Text' in response.get('response', {}):
                print(f"   💬 Text: {response['response']['Text']['message']}")
            elif 'Interactive' in response.get('response', {}):
                interactive = response['response']['Interactive']
                print(f"   🔘 Interactive: {interactive['body_text']}")
                print(f"   🔳 Buttons: {[btn['title'] for btn in interactive['buttons']]}")
            
            print("   " + "="*50)
            
    except KeyboardInterrupt:
        print("\n👋 Monitoring stopped")
    finally:
        consumer.close()

if __name__ == "__main__":
    monitor_output()
