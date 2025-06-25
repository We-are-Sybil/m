# Webhook Notification Payload Examples

## Received Messages

### Text Messages

The following is an example of a text message you received from a customer:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "<WHATSAPP_BUSINESS_ACCOUNT_ID>",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "<BUSINESS_DISPLAY_PHONE_NUMBER>",
              "phone_number_id": "<BUSINESS_PHONE_NUMBER_ID>"
            },
            "contacts": [
              {
                "profile": {
                  "name": "<WHATSAPP_USER_NAME>"
                },
                "wa_id": "<WHATSAPP_USER_ID>"
              }
            ],
            "messages": [
              {
                "from": "<WHATSAPP_USER_PHONE_NUMBER>",
                "id": "<WHATSAPP_MESSAGE_ID>",
                "timestamp": "<WEBHOOK_SENT_TIMESTAMP>",
                "text": {
                  "body": "<MESSAGE_BODY_TEXT>"
                },
                "type": "text"
              }
            ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Reaction Messages

The following is an example of a reaction message you received from a customer. You will not receive this webbook if the message the customer is reacting to is more than 30 days old.

```json
{
"object": "whatsapp_business_account",
"entry": [{
    "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
    "changes": [{
        "value": {
            "messaging_product": "whatsapp",
            "metadata": {
                "display_phone_number": PHONE_NUMBER,
                "phone_number_id": PHONE_NUMBER_ID
            },
            "contacts": [{
                "profile": {
                  "name": "NAME"
                },
                "wa_id": PHONE_NUMBER
              }],
            "messages": [{
                "from": PHONE_NUMBER,
                "id": "wamid.ID",
                "timestamp": TIMESTAMP,
                "reaction": {
                  "message_id": "MESSAGE_ID",
                  "emoji": "EMOJI"
                },
                "type": "reaction"
              }]
        },
        "field": "messages"
      }]
}]
}
```

Note that for reactions, the `timestamp` value indicates when the customer sent the reaction, not when the webhook was generated.

## Media Messages

When a message with media is received, the WhatsApp Business Platform downloads the media. A notification is sent to your Webhook once the media is downloaded.

The Webhook notification contains information that identifies the media object and enables you to find and retrieve the object. Use the media endpoints to retrieve the media.

```json
{
  "object": "whatsapp_business_account",
  "entry": [{
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [{
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                  "display_phone_number": PHONE_NUMBER,
                  "phone_number_id": PHONE_NUMBER_ID
              },
              "contacts": [{
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "WHATSAPP_ID"
                }],
              "messages": [{
                  "from": PHONE_NUMBER,
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "type": "image",
                  "image": {
                    "caption": "CAPTION",
                    "mime_type": "image/jpeg",
                    "sha256": "IMAGE_HASH",
                    "id": "ID"
                  }
                }]
          },
          "field": "messages"
        }]
    }]
}
```
When you receive a sticker, you will get the following notification:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "ID",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "PHONE_NUMBER",
              "phone_number_id": "PHONE_NUMBER_ID"
            },
            "contacts": [
              {
                "profile": {
                  "name": "NAME"
                },
                "wa_id": "ID"
              }
            ],
            "messages": [
              {
                "from": "SENDER_PHONE_NUMBER",
                "id": "wamid.ID",
                "timestamp": "TIMESTAMP",
                "type": "sticker",
                "sticker": {
                  "mime_type": "image/webp",
                  "sha256": "HASH",
                  "id": "ID"
                }
              }
            ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Unknown Message 

It's possible to receive an unknown message callback notification. For example, a customer could send you a message that's not supported, such as a disappearing message (in which case we'd notify the customer that the message type is not supported), or a business is using the API to send a template message to another business.

The following is an example of a message you received from a customer that is not supported.

```json
{
  "object": "whatsapp_business_account",
  "entry": [{
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [{
          "value": {
              "messaging_product": "whatsapp",
              "metadata": { 
                "display_phone_number": "PHONE_NUMBER", 
                "phone_number_id": "PHONE_NUMBER_ID" 
              },
              "contacts": [{
                  "profile": { 
                    "name": "NAME" 
                  }, 
                  "wa_id": "WHATSAPP_ID"
                }],
              "messages": [{
                  "from": "PHONE_NUMBER",
                  "id": "wamid.ID", 
                  "timestamp": "TIMESTAMP",
                  "errors": [ 
                    { 
                      "code": 131051, 
                      "details": "Message type is not currently supported",
                      "title": "Unsupported message type"
                    }],
                   "type": "unknown"
                   }]
            }
            "field": "messages"
        }],
    }]
}
```

## Location Messages

The following is an example of a location message you received from a customer:

```json
{
  "object": "whatsapp_business_account",
  "entry": [{
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [{
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                  "display_phone_number": "PHONE_NUMBER",
                  "phone_number_id": "PHONE_NUMBER_ID"
              },
              "contacts": [{
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "WHATSAPP_ID"
                }],
              "messages": [{
                  "from": "PHONE_NUMBER",
                  "id": "wamid.ID",
                  "timestamp": "TIMESTAMP",
                 "location": {
                    "latitude": LOCATION_LATITUDE,
                    "longitude": LOCATION_LONGITUDE,
                    "name": LOCATION_NAME,
                    "address": LOCATION_ADDRESS,
                 }
                }]
          },
          "field": "messages"
        }]
    }]
}
```
## Contact Messages

The following is an example of a contact message you received from a customer:

```json
{
  "object":"whatsapp_business_account",
  "entry":[{
    "id":"WHATSAPP_BUSINESS_ACCOUNT_ID",
    "changes":[{
      "value":{
        "messaging_product":"whatsapp",
        "metadata": {
          "display_phone_number":"PHONE_NUMBER",
          "phone_number_id":"PHONE_NUMBER_ID"
          },
        "contacts": [{
          "profile":{
            "name":"NAME"
            },
          "wa_id":"WHATSAPP_ID"
          }],
        "messages":[{
          "from":"PHONE_NUMBER",
          "id":"wamid.ID",
          "timestamp":"TIMESTAMP",
          "contacts":[{
            "addresses":[{
              "city":"CONTACT_CITY",
              "country":"CONTACT_COUNTRY",
              "country_code":"CONTACT_COUNTRY_CODE",
              "state":"CONTACT_STATE",
              "street":"CONTACT_STREET",
              "type":"HOME or WORK",
              "zip":"CONTACT_ZIP"
            }],
            "birthday":"CONTACT_BIRTHDAY",
            "emails":[{
              "email":"CONTACT_EMAIL",
              "type":"WORK or HOME"
              }],
            "name":{
              "formatted_name":"CONTACT_FORMATTED_NAME",
              "first_name":"CONTACT_FIRST_NAME",
              "last_name":"CONTACT_LAST_NAME",
              "middle_name":"CONTACT_MIDDLE_NAME",
              "suffix":"CONTACT_SUFFIX",
              "prefix":"CONTACT_PREFIX"
              },
            "org":{
              "company":"CONTACT_ORG_COMPANY",
              "department":"CONTACT_ORG_DEPARTMENT",
              "title":"CONTACT_ORG_TITLE"
              },
            "phones":[{
              "phone":"CONTACT_PHONE",
              "wa_id":"CONTACT_WA_ID",
              "type":"HOME or WORK>"
              }],
            "urls":[{
              "url":"CONTACT_URL",
              "type":"HOME or WORK"
              }]
            }]
          }]
        },
      "field":"messages"
    }]
  }]
}
```

## Received Callback from a Quick Reply Button

When your customer clicks on a quick reply button in an interactive message template, a response is sent. Below is an example of the callback format.

```json
{
  "object": "whatsapp_business_account",
  "entry": [{
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [{
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                  "display_phone_number": PHONE_NUMBER,
                  "phone_number_id": PHONE_NUMBER_ID
              },
              "contacts": [{
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "WHATSAPP_ID"
                }],
              "messages": [{
                  "context": {
                    "from": PHONE_NUMBER,
                    "id": "wamid.ID"
                  },
                  "from": "16315551234",
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "type": "button",
                  "button": {
                    "text": "No",
                    "payload": "No-Button-Payload"
                  }
                }]
          },
          "field": "messages"
        }]
    }]
}
```

## Received Answer From List Message

The following webhook notification is received when a user clicks on an item from a list message you sent:

```json
{
  "object": "whatsapp_business_account",
  "entry": [{
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [{
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                  "display_phone_number": PHONE_NUMBER,
                  "phone_number_id": PHONE_NUMBER_ID
              },
              "contacts": [{
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "WHATSAPP_ID"
                }],
              "messages": [{
                  "context": {
                    "from": PHONE_NUMBER,
                    "id": "wamid.ID"
                  },
                  "from": "16315551234",
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "type": "button",
                  "button": {
                    "text": "No",
                    "payload": "No-Button-Payload"
                  }
                }]
          },
          "field": "messages"
        }]
    }]
}
```

## Received Answer From List Message

The following webhook notification is received when a user clicks on an item from a list message you sent:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [
        {
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                   "display_phone_number": "PHONE_NUMBER",
                   "phone_number_id": "PHONE_NUMBER_ID",
              },
              "contacts": [
                {
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "PHONE_NUMBER_ID"
                }
              ],
              "messages": [
                {
                  "from": PHONE_NUMBER_ID,
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "interactive": {
                    "list_reply": {
                      "id": "list_reply_id",
                      "title": "list_reply_title",
                      "description": "list_reply_description"
                    },
                    "type": "list_reply"
                  },
                  "type": "interactive"
                }
              ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Answer to Reply Button

The following webhook notification is received when a user clicks on a reply button you sent:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [
        {
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                   "display_phone_number": "PHONE_NUMBER",
                   "phone_number_id": PHONE_NUMBER_ID,
              },
              "contacts": [
                {
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "PHONE_NUMBER_ID"
                }
              ],
              "messages": [
                {
                  "from": PHONE_NUMBER_ID,
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "interactive": {
                    "button_reply": {
                      "id": "unique-button-identifier-here",
                      "title": "button-text",
                    },
                    "type": "button_reply"
                  },
                  "type": "interactive"
                }
              ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Message Triggered by Click to WhatsApp Ads

You get the following webhook when a conversation is started after a user clicks an ad with a Click to WhatsApp’s call-to-action:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "ID",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "PHONE_NUMBER",
              "phone_number_id": "PHONE_NUMBER_ID"
            },
            "contacts": [
              {
                "profile": {
                  "name": "NAME"
                },
                "wa_id": "ID"
              }
            ],
            "messages": [
              {
                "referral": {
                  "source_url": "AD_OR_POST_FB_URL",
                  "source_id": "ADID",
                  "source_type": "ad or post",
                  "headline": "AD_TITLE",
                  "body": "AD_DESCRIPTION",
                  "media_type": "image or video",
                  "image_url": "RAW_IMAGE_URL",
                  "video_url": "RAW_VIDEO_URL",
                  "thumbnail_url": "RAW_THUMBNAIL_URL",
                  "ctwa_clid": "CTWA_CLID"
                },
                "from": "SENDER_PHONE_NUMBERID",
                "id": "wamid.ID",
                "timestamp": "TIMESTAMP",
                "type": "text",
                "text": {
                  "body": "BODY"
                }
              }
            ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Answer to Reply Button

The following webhook notification is received when a user clicks on a reply button you sent:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [
        {
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                   "display_phone_number": "PHONE_NUMBER",
                   "phone_number_id": PHONE_NUMBER_ID,
              },
              "contacts": [
                {
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "PHONE_NUMBER_ID"
                }
              ],
              "messages": [
                {
                  "from": PHONE_NUMBER_ID,
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "interactive": {
                    "button_reply": {
                      "id": "unique-button-identifier-here",
                      "title": "button-text",
                    },
                    "type": "button_reply"
                  },
                  "type": "interactive"
                }
              ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Message Triggered by Click to WhatsApp Ads

You get the following webhook when a conversation is started after a user clicks an ad with a Click to WhatsApp’s call-to-action:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "ID",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "PHONE_NUMBER",
              "phone_number_id": "PHONE_NUMBER_ID"
            },
            "contacts": [
              {
                "profile": {
                  "name": "NAME"
                },
                "wa_id": "ID"
              }
            ],
            "messages": [
              {
                "referral": {
                  "source_url": "AD_OR_POST_FB_URL",
                  "source_id": "ADID",
                  "source_type": "ad or post",
                  "headline": "AD_TITLE",
                  "body": "AD_DESCRIPTION",
                  "media_type": "image or video",
                  "image_url": "RAW_IMAGE_URL",
                  "video_url": "RAW_VIDEO_URL",
                  "thumbnail_url": "RAW_THUMBNAIL_URL",
                  "ctwa_clid": "CTWA_CLID"
                },
                "from": "SENDER_PHONE_NUMBERID",
                "id": "wamid.ID",
                "timestamp": "TIMESTAMP",
                "type": "text",
                "text": {
                  "body": "BODY"
                }
              }
            ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Answer From List Message

The following webhook notification is received when a user clicks on an item from a list message you sent:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [
        {
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                   "display_phone_number": "PHONE_NUMBER",
                   "phone_number_id": "PHONE_NUMBER_ID",
              },
              "contacts": [
                {
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "PHONE_NUMBER_ID"
                }
              ],
              "messages": [
                {
                  "from": PHONE_NUMBER_ID,
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "interactive": {
                    "list_reply": {
                      "id": "list_reply_id",
                      "title": "list_reply_title",
                      "description": "list_reply_description"
                    },
                    "type": "list_reply"
                  },
                  "type": "interactive"
                }
              ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Answer to Reply Button

The following webhook notification is received when a user clicks on a reply button you sent:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "WHATSAPP_BUSINESS_ACCOUNT_ID",
      "changes": [
        {
          "value": {
              "messaging_product": "whatsapp",
              "metadata": {
                   "display_phone_number": "PHONE_NUMBER",
                   "phone_number_id": PHONE_NUMBER_ID,
              },
              "contacts": [
                {
                  "profile": {
                    "name": "NAME"
                  },
                  "wa_id": "PHONE_NUMBER_ID"
                }
              ],
              "messages": [
                {
                  "from": PHONE_NUMBER_ID,
                  "id": "wamid.ID",
                  "timestamp": TIMESTAMP,
                  "interactive": {
                    "button_reply": {
                      "id": "unique-button-identifier-here",
                      "title": "button-text",
                    },
                    "type": "button_reply"
                  },
                  "type": "interactive"
                }
              ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```

## Received Message Triggered by Click to WhatsApp Ads

You get the following webhook when a conversation is started after a user clicks an ad with a Click to WhatsApp’s call-to-action:

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "ID",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "PHONE_NUMBER",
              "phone_number_id": "PHONE_NUMBER_ID"
            },
            "contacts": [
              {
                "profile": {
                  "name": "NAME"
                },
                "wa_id": "ID"
              }
            ],
            "messages": [
              {
                "referral": {
                  "source_url": "AD_OR_POST_FB_URL",
                  "source_id": "ADID",
                  "source_type": "ad or post",
                  "headline": "AD_TITLE",
                  "body": "AD_DESCRIPTION",
                  "media_type": "image or video",
                  "image_url": "RAW_IMAGE_URL",
                  "video_url": "RAW_VIDEO_URL",
                  "thumbnail_url": "RAW_THUMBNAIL_URL",
                  "ctwa_clid": "CTWA_CLID"
                },
                "from": "SENDER_PHONE_NUMBERID",
                "id": "wamid.ID",
                "timestamp": "TIMESTAMP",
                "type": "text",
                "text": {
                  "body": "BODY"
                }
              }
            ]
          },
          "field": "messages"
        }
      ]
    }
  ]
}
```
