#!/bin/bash

# create-topics.sh - A script to create Kafka topics.

BOOTSTRAP_SERVER="m-kafka-1:19092"
echo "--- Waiting for Kafka broker at $BOOTSTRAP_SERVER to be available... ---"

# A simple wait loop
until /opt/kafka/bin/kafka-topics.sh --bootstrap-server $BOOTSTRAP_SERVER --list; do
  echo "Kafka broker is not available yet. Retrying in 5 seconds..."
  sleep 5
done

echo "--- Kafka broker is ready. Proceeding with topic creation... ---"

# Function to create a Kafka topic with specified parameters.
# Usage: create_topic <topic_name> <partitions> <replication_factor> <retention_minutes>
create_topic() {
    TOPIC_NAME=$1
    PARTITIONS=$2
    REPLICATION_FACTOR=$3
    RETENTION_MINUTES=$4
    # Convert minutes to milliseconds for Kafka's configuration
    RETENTION_MS=$((RETENTION_MINUTES * 60 * 1000))

    if /opt/kafka/bin/kafka-topics.sh --bootstrap-server $BOOTSTRAP_SERVER --describe --topic $TOPIC_NAME >/dev/null 2>&1; then
        echo "Topic '$TOPIC_NAME' already exists. Skipping creation."
    else
        echo "Creating topic '$TOPIC_NAME' with $PARTITIONS partitions, RF $REPLICATION_FACTOR, and retention ${RETENTION_MINUTES} minutes (${RETENTION_MS}ms)."
        /opt/kafka/bin/kafka-topics.sh --bootstrap-server $BOOTSTRAP_SERVER \
            --create \
            --topic $TOPIC_NAME \
            --partitions $PARTITIONS \
            --replication-factor $REPLICATION_FACTOR \
            --config retention.ms=$RETENTION_MS
    fi
}

# --- Define retention periods in minutes ---
RETENTION_7_DAYS=$((7 * 24 * 60))  # 10080 minutes
RETENTION_30_DAYS=$((30 * 24 * 60))  # 43200 minutes
RETENTION_1_DAY=$((1 * 24 * 60))  # 1440 minutes
RETENTION_90_DAYS=$((90 * 24 * 60))  # 129600 minutes

# --- Core conversation events ---
create_topic "conversation.messages" 3 1 $RETENTION_7_DAYS
create_topic "conversation.interactions" 2 1 $RETENTION_7_DAYS
create_topic "conversation.responses" 2 1 $RETENTION_7_DAYS
create_topic "conversation.failures" 1 1 $RETENTION_30_DAYS

# --- Retry mechanisms ---
create_topic "conversation.messages.retry" 2 1 $RETENTION_1_DAY
create_topic "conversation.interactions.retry" 1 1 $RETENTION_1_DAY

# --- Dead letter queues ---
create_topic "conversation.messages.dlq" 1 1 $RETENTION_90_DAYS
create_topic "conversation.interactions.dlq" 1 1 $RETENTION_90_DAYS

# --- System and monitoring topics ---
create_topic "system.metrics" 1 1 $RETENTION_7_DAYS
create_topic "system.health" 1 1 $RETENTION_1_DAY

echo "--- Topic creation script finished. ---"
