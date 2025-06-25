# Audio Messages

Audio messages display an audio icon and a link to an audio file. When the WhatsApp user taps the icon, the WhatsApp client loads and plays the audio file.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send an audio message to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "audio",
  "audio": {
    "id": "<MEDIA_ID>", <!-- Only if using uploaded media -->
    "link": "<MEDIA_URL>" <!-- Only if using hosted media (not recommended) -->
  }
}'
```

## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                                                               | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                                                   | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                                                | v23\.0                                                                                                                                                                                                                                         |
| <MEDIA\_ID> String                             | Required if using uploaded media, otherwise omit\. ID of the uploaded media asset\.                                                                                                           | 1013859600285441                                                                                                                                                                                                                               |
| <MEDIA\_URL> String                            | Required if using hosted media, otherwise omit\. URL of the media asset hosted on your public server\. For better performance, we recommend using id and an uploaded media asset ID instead\. | https://www\.luckyshrub\.com/media/ringtones/wind\-chime\.mp3                                                                                                                                                                                  |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                                                | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                                                                       | \+16505551234                                                                                                                                                                                                                                  |

## Supported audio formats

| Audio Type | Extension | MIME Type            | Max Size |
|------------|-----------|----------------------|----------|
| AAC        | .aac      | audio/aac            | 16 MB    |
| AMR        | .amr      | audio/amr            | 16 MB    |
| MP3        | .mp3      | audio/mpeg           | 16 MB    |
| MP4 Audio  | .mp4      | audio/mp4            | 16 MB    |
| OGG Audio  | .ogg      | audio/ogg (OPUS codecs only; base audio/ogg not supported; mono input only)           | 16 MB    |

## Example request

Example request to send an image message using an uploaded media ID and a caption.

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "audio",
  "audio": {
    "id" : "1013859600285441"
  }
}'
```

## Example Response

```json
Example Response
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Contacts Messages

Contacts messages allow you to send rich contact information directly to WhatsApp users, such as names, phone numbers, physical addresses, and email addresses.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send a contacts message to a WhatsApp user.


```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "contacts",
  "contacts": [
    {
      "addresses": [
        {
          "street": "<STREET_NUMBER_AND_NAME>",
          "city": "<CITY>",
          "state": "<STATE_CODE>",
          "zip": "<ZIP_CODE>",
          "country": "<COUNTRY_NAME>",
          "country_code": "<COUNTRY_CODE>",
          "type": "<ADDRESS_TYPE>"
        }
        <!-- Additional addresses objects go here, if using -->
      ],
      "birthday": "<BIRTHDAY>",
      "emails": [
        {
          "email": "<EMAIL_ADDRESS>",
          "type": "<EMAIL_TYPE>"
        }
        <!-- Additional emails objects go here, if using -->
      ],
      "name": {
        "formatted_name": "<FULL_NAME>",
        "first_name": "<FIRST_NAME>",
        "last_name": "<LAST_NAME>",
        "middle_name": "<MIDDLE_NAME>",
        "suffix": "<SUFFIX>",
        "prefix": "<PREFIX>"
      },
      "org": {
        "company": "<COMPANY_OR_ORG_NAME>",
        "department": "<DEPARTMENT_NAME>",
        "title": "<JOB_TITLE>"
      },
      "phones": [
          "phone": "<PHONE_NUMBER>",
          "type": "<PHONE_NUMBER_TYPE>",
          "wa_id": "<WHATSAPP_USER_ID>"
        }
        <!-- Additional phones objects go here, if using -->
      ],
      "urls": [
        {
          "url": "<WEBSITE_URL>",
          "type": "<WEBSITE_TYPE>"
        }
        <!-- Additional URLs go here, if using -->
      ]
    }
  ]
}'
```

## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                               | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                   | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <ADDRESS\_TYPE> String                         | Optional\. Type of address, such as home or work\.                                                                                                            | Home                                                                                                                                                                                                                                           |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                | v23\.0                                                                                                                                                                                                                                         |
| <BIRTHDAY> String                              | Optional\. Contact's birthday\. Must be in YYYY\-MM\-DD format\.                                                                                              | 1999\-01\-23                                                                                                                                                                                                                                   |
| <CITY> String                                  | Optional\. City where the contact resides\.                                                                                                                   | Menlo Park                                                                                                                                                                                                                                     |
| <COMPANY\_OR\_ORG\_NAME> String                | Optional\. Name of the company where the contact works\.                                                                                                      | Lucky Shrub                                                                                                                                                                                                                                    |
| <COUNTRY\_CODE> String                         | Optional\. ISO two\-letter country code\.                                                                                                                     | US                                                                                                                                                                                                                                             |
| <COUNTRY\_NAME> String                         | Optional\. Country name\.                                                                                                                                     | United States                                                                                                                                                                                                                                  |
| <DEPARTMENT\_NAME> String                      | Optional\. Department within the company\.                                                                                                                    | Legal                                                                                                                                                                                                                                          |
| <EMAIL\_ADDRESS> String                        | Optional\. Email address of the contact\.                                                                                                                     | bjohnson@luckyshrub\.com                                                                                                                                                                                                                       |
| <EMAIL\_TYPE> String                           | Optional\. Type of email, such as personal or work\.                                                                                                          | Work                                                                                                                                                                                                                                           |
| <FIRST\_NAME> String                           | Optional\. Contact's first name\.                                                                                                                             | Barbara                                                                                                                                                                                                                                        |
| <FORMATTED\_NAME> String                       | Required\. Contact's formatted name\. This will appear in the message alongside the profile arrow button\.                                                    | Barbara J\. Johnson                                                                                                                                                                                                                            |
| <JOB\_TITLE> String                            | Optional\. Contact's job title\.                                                                                                                              | Lead Counsel                                                                                                                                                                                                                                   |
| <LAST\_NAME> String                            | Optional\. Contact's last name\.                                                                                                                              | Johnson                                                                                                                                                                                                                                        |
| <MIDDLE\_NAME> String                          | Optional\. Contact's middle name\.                                                                                                                            | Joana                                                                                                                                                                                                                                          |
| <PHONE\_NUMBER> String                         | Optional\. WhatsApp user phone number\.                                                                                                                       | \+16505559999                                                                                                                                                                                                                                  |
| <PHONE\_NUMBER\_TYPE> String                   | Optional\. Type of phone number\. For example, cell, mobile, main, iPhone, home, work, etc\.                                                                  | Home                                                                                                                                                                                                                                           |
| <PREFIX> String                                | Optional\. Prefix for the contact's name, such as Mr\., Ms\., Dr\., etc\.                                                                                     | Dr\.                                                                                                                                                                                                                                           |
| <STATE\_CODE> String                           | Optional\. Two\-letter state code\.                                                                                                                           | CA                                                                                                                                                                                                                                             |
| <STREET\_NUMBER\_AND\_NAME> String             | Optional\. Street address of the contact\.                                                                                                                    | 1 Lucky Shrub Way                                                                                                                                                                                                                              |
| <SUFFIX> String                                | Optional\. Suffix for the contact's name, if applicable\.                                                                                                     | Esq\.                                                                                                                                                                                                                                          |
| <WEBSITE\_TYPE> String                         | Optional\. Type of website\. For example, company, work, personal, Facebook Page, Instagram, etc\.                                                            | Company                                                                                                                                                                                                                                        |
| <WEBSITE\_URL> String                          | Optional\. Website URL associated with the contact or their company\.                                                                                         | https://www\.luckyshrub\.com                                                                                                                                                                                                                   |
| <WHATSAPP\_USER\_ID> String                    | Optional\. WhatsApp user ID\. If omitted, the message will display an Invite to WhatsApp button instead of the standard buttons\. See Button Behavior below\. | 19175559999                                                                                                                                                                                                                                    |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                                       | \+16505551234                                                                                                                                                                                                                                  |
| <ZIP\_CODE> String                             | Optional\. Postal or ZIP code\.                                                                                                                               | 94025                                                                                                                                                                                                                                          |

## Button behavior

If you include the contact's WhatsApp ID in the message (via the `wa_id` property), the message will include a Message and a Save contact button.

If the WhatsApp user taps the **Message** button, it will open a new message with the contact. If the user taps the **Save contact** button, they will be given the option to save the contact as a new contact, or to update an existing contact.

If you omit the `wa_id` property, both buttons will be replaced with an `Invite to WhatsApp` button.


## Example request

Example request to send a contacts message with two physical addresses, two email addresses, two phone numbers, and two website URLs.


```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "to": "+16505551234",
  "type": "contacts",
  "contacts": [
    {
      "addresses": [
        {
          "street": "1 Lucky Shrub Way",
          "city": "Menlo Park",
          "state": "CA",
          "zip": "94025",
          "country": "United States",
          "country_code": "US",
          "type": "Office"
        },
        {
          "street": "1 Hacker Way",
          "city": "Menlo Park",
          "state": "CA",
          "zip": "94025",
          "country": "United States",
          "country_code": "US",
          "type": "Pop-Up"
        }
      ],
      "birthday": "1999-01-23",
      "emails": [
        {
          "email": "bjohnson@luckyshrub.com",
          "type": "Work"
        },
        {
          "email": "bjohnson@luckyshrubplants.com",
          "type": "Work (old)"
        }
      ],
      "name": {
        "formatted_name": "Barbara J. Johnson",
        "first_name": "Barbara",
        "last_name": "Johnson",
        "middle_name": "Joana",
        "suffix": "Esq.",
        "prefix": "Dr."
      },
      "org": {
        "company": "Lucky Shrub",
        "department": "Legal",
        "title": "Lead Counsel"
      },
      "phones": [
        {
          "phone": "+16505559999",
          "type": "Landline"
        },
        {
          "phone": "+19175559999",
          "type": "Mobile",
          "wa_id": "19175559999"
        }
      ],
      "urls": [
        {
          "url": "https://www.luckyshrub.com",
          "type": "Company"
        },
        {
          "url": "https://www.facebook.com/luckyshrubplants",
          "type": "Company (FB)"
        }
      ]
    }
  ]
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Document Messages

Document messages display a document icon, linked to a document that a WhatsApp user can tap to download.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send a document message to a WhatsApp user.


```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "document",
  "document": {
    "id": "<MEDIA_ID>", <!-- Only if using uploaded media -->
    "link": "<MEDIA_URL>", <!-- Only if using hosted media (not recommended) -->
    "caption": "<MEDIA_CAPTION_TEXT>",
    "filename": "<MEDIA_FILENAME>",
    "caption": "<MEDIA_CAPTION_TEXT>"
  }
}'
```


## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                                                               | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                                                   | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                                                | v23\.0                                                                                                                                                                                                                                         |
| <MEDIA\_CAPTION\_TEXT> String                  | Optional\. Media asset caption text\. Maximum 1024 characters\.                                                                                                                               | Lucky Shrub Invoice                                                                                                                                                                                                                            |
| <MEDIA\_FILENAME> String                       | Optional\. Document filename, with extension\. The WhatsApp client will use an appropriate file type icon based on the extension\.                                                            | lucky\-shrub\-invoice\.pdf                                                                                                                                                                                                                     |
| <MEDIA\_ID> String                             | Required if using uploaded media, otherwise omit\. ID of the uploaded media asset\.                                                                                                           | 1013859600285441                                                                                                                                                                                                                               |
| <MEDIA\_URL> String                            | Required if using hosted media, otherwise omit\. URL of the media asset hosted on your public server\. For better performance, we recommend using id and an uploaded media asset ID instead\. | https://www\.luckyshrub\.com/invoices/FmOzfD9cKf/lucky\-shrub\-invoice\.pdf                                                                                                                                                                    |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                                                | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                                                                       | \+16505551234                                                                                                                                                                                                                                  |
## Supported document formats

| **Document Type**    | **Extension** | **MIME Type**                                                                 | **Max Size** |
|----------------------|---------------|-------------------------------------------------------------------------------|--------------|
| Text                 | \.txt         | text/plain                                                                    | 100 MB       |
| Microsoft Excel      | \.xls         | application/vnd\.ms\-excel                                                    | 100 MB       |
| Microsoft Excel      | \.xlsx        | application/vnd\.openxmlformats\-officedocument\.spreadsheetml\.sheet         | 100 MB       |
| Microsoft Word       | \.doc         | application/msword                                                            | 100 MB       |
| Microsoft Word       | \.docx        | application/vnd\.openxmlformats\-officedocument\.wordprocessingml\.document   | 100 MB       |
| Microsoft PowerPoint | \.ppt         | application/vnd\.ms\-powerpoint                                               | 100 MB       |
| Microsoft PowerPoint | \.pptx        | application/vnd\.openxmlformats\-officedocument\.presentationml\.presentation | 100 MB       |
| PDF                  | \.pdf         | application/pdf                                                               | 100 MB       |




## Example request

Example request to send a PDF in a document message with a caption to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "document",
  "document": {
    "id": "1376223850470843",
    "filename": "order_abc123.pdf",
    "caption": "Your order confirmation (PDF)"
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Image Messages

Image messages display a single image and an optional caption.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send a image message to a WhatsApp user.


```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "image",
  "image": {
    "id": "<MEDIA_ID>", <!-- Only if using uploaded media -->
    "link": "<MEDIA_URL>", <!-- Only if using hosted media (not recommended) -->
    "caption": "<MEDIA_CAPTION_TEXT>"
  }
}'
```


## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                                                               | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                                                   | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                                                | v23\.0                                                                                                                                                                                                                                         |
| <MEDIA\_CAPTION\_TEXT> String                  | Optional\. Media asset caption text\. Maximum 1024 characters\.                                                                                                                               | The best succulent ever?                                                                                                                                                                                                                       |
| <MEDIA\_ID> String                             | Required if using uploaded media, otherwise omit\. ID of the uploaded media asset\.                                                                                                           | 1\.01385960028544E\+15                                                                                                                                                                                                                         |
| <MEDIA\_URL> String                            | Required if using hosted media, otherwise omit\. URL of the media asset hosted on your public server\. For better performance, we recommend using id and an uploaded media asset ID instead\. | https://www\.luckyshrub\.com/assets/succulents/aloe\.png                                                                                                                                                                                       |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                                                | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                                                                       | +16505551234                                                                                                                                                                                                                                    |

## Supported document formats


| **Image Type** | **Extension** | **MIME Type** | **Max Size** |
|----------------|---------------|---------------|--------------|
| JPEG           | .jpg          | image/jpeg    | 5 MB         |
| PNG            | .png          | image/png     | 5 MB         |

## Example request

Example request to send an image message with a caption to a WhatsApp user.


```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "image",
  "image": {
    "id" : "1479537139650973",
    "caption": "The best succulent ever?"
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Interactive Call-to-Action URL Button Messages

WhatsApp users may be hesitant to tap raw URLs containing lengthy or obscure strings in text messages. In these situations, you may wish to send an interactive call-to-action (CTA) URL button message instead. CTA URL button messages allow you to map any URL to a button so you don't have to include the raw URL in the message body.

## Request syntax

Endpoint: `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages`



```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "interactive",
  "interactive": {
    "type": "cta_url",

    <!-- If using document header, otherwise omit -->
    "header": {
      "type": "document",
      "document": {
        "link": "<ASSET_URL>"
      }
    },

    <!-- If using image header, otherwise omit -->
    "header": {
      "type": "image",
      "image": {
        "link": "<ASSET_URL>"
      }
    },

    <!-- If using text header, otherwise omit -->
    "header": {
      "type": "text",
      "text": "<HEADER_TEXT>"
      }
    },

    <!-- If using video header, otherwise omit -->
    "header": {
      "type": "video",
      "video": {
        "link": "<ASSET_URL>"
      }
    },

    "body": {
      "text": "<BODY_TEXT>"
    },
    "action": {
      "name": "cta_url",
      "parameters": {
        "display_text": "<BUTTON_LABEL_TEXT>",
        "url": "<BUTTON_URL>"
      }
    },

    <!-- If using footer text, otherwise omit -->
    "footer": {
      "text": "<FOOTER_TEXT>"
    }
  }
}'
```

## Request parameters

| **Placeholder**                                | **Description**                                                                                         | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                             | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                          | v23\.0                                                                                                                                                                                                                                         |
| <ASSET\_URL> String                            | Required if using a header with a media asset\. Asset URL on a public server\.                          | https://www\.luckyshrub\.com/assets/lucky\-shrub\-banner\-logo\-v1\.png                                                                                                                                                                        |
| <BODY\_TEXT> String                            | Required\. Body text\. URLs are automatically hyperlinked\. Maximum 1024 characters\.                   | Tap the button below to see available dates\.                                                                                                                                                                                                  |
| <BUTTON\_LABEL\_TEXT> String                   | Required\. Button label text\. Must be unique if using multiple buttons\. Maximum 20 characters\.       | See Dates                                                                                                                                                                                                                                      |
| <BUTTON\_URL>                                  | Required\. URL to load in the device's default web browser when tapped by the WhatsApp user\.           | https://www\.luckyshrub\.com?clickID=kqDGWd24Q5TRwoEQTICY7W1JKoXvaZOXWAS7h1P76s0R7Paec4                                                                                                                                                        |
| <FOOTER\_TEXT> String                          | Required if using a footer\. Footer text\. URLs are automatically hyperlinked\. Maximum 60 characters\. | Dates subject to change\.                                                                                                                                                                                                                      |
| <HEADER\_TEXT> String                          | Required if using a text header\. Header text\. Maximum 60 characters\.                                 | New workshop dates announced\!                                                                                                                                                                                                                 |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                          | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                 | \+16505551234                                                                                                                                                                                                                                  |

## Example request

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "interactive",
  "interactive": {
    "type": "cta_url",
    "header": {
      "type": "image",
      "image": {
        "link": "https://www.luckyshrub.com/assets/lucky-shrub-banner-logo-v1.png"
      }
    },
    "body": {
      "text": "Tap the button below to see available dates."
    },
    "action": {
      "name": "cta_url",
      "parameters": {
        "display_text": "See Dates",
        "url": "https://www.luckyshrub.com?clickID=kqDGWd24Q5TRwoEQTICY7W1JKoXvaZOXWAS7h1P76s0R7Paec4"
      }
    },
    "footer": {
      "text": "Dates subject to change."
    }
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Interactive List Messages

Interactive list messages allow you to present WhatsApp users with a list of options to choose from (options are defined as rows in the request payload). When a user taps the button in the message, it displays a modal that lists the options available. Users can then choose one option and their selection will be sent as a reply.This triggers a webhook, which identifies the option selected by the user. Interactive list messages support up to 10 sections, with up to 10 rows for all sections combined, and can include an optional header and footer.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send an interactive list message to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "interactive",
  "interactive": {
    "type": "list",
    "header": {
      "type": "text",
      "text": "<MESSAGE_HEADER_TEXT>"
    },
    "body": {
      "text": "<MESSAGE_BODY_TEXT>"
    },
    "footer": {
      "text": "<MESSAGE_FOOTER_TEXT>"
    },
    "action": {
      "button": "<BUTTON_TEXT>",
      "sections": [
        {
          "title": "<SECTION_TITLE_TEXT>",
          "rows": [
            {
              "id": "<ROW_ID>",
              "title": "<ROW_TITLE_TEXT>",
              "description": "<ROW_DESCRIPTION_TEXT>"
            }
            <!-- Additional rows would go here -->
          ]
        }
        <!-- Additional sections would go here -->
      ]
    }
  }
}'
```

## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                                                                                        | **Sample Value**                                                                                                                                                                                                                               |
|------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                                                                            | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                                                                         | v23\.0                                                                                                                                                                                                                                         |
| <BUTTON\_TEXT> String                          | Required\. Button label text\. When tapped, reveals rows \(options the WhatsApp user can tap\)\. Supports a single button\. Maximum 20 characters\.                                                                    | Shipping Options                                                                                                                                                                                                                               |
| <MESSAGE\_BODY\_TEXT> String                   | Required\. Message body text\. Supports URLs\. Maximum 4096 characters\.                                                                                                                                               | Which shipping option do you prefer?                                                                                                                                                                                                           |
| <MESSAGE\_FOOTER\_TEXT> String                 | Optional\. Message footer text\. Maximum 60 characters\.                                                                                                                                                               | Lucky Shrub: Your gateway to succulents™                                                                                                                                                                                                       |
| <MESSAGE\_HEADER\_TEXT> String                 | Optional\. The header object is optional\. Supports text header type only\. Maximum 60 characters\.                                                                                                                    | Choose Shipping Option                                                                                                                                                                                                                         |
| <ROW\_DESCRIPTION\_TEXT> String                | Optional\. Row description\. Maximum 72 characters\.                                                                                                                                                                   | Next Day to 2 Days                                                                                                                                                                                                                             |
| <ROW\_ID> String                               | Required\. Arbitrary string identifying the row\. This ID will be included in the webhook payload if the user submits the selection\. At least one row is required\. Supports up to 10 rows\. Maximum 200 characters\. | priority\_express                                                                                                                                                                                                                              |
| <ROW\_TITLE\_TEXT> String                      | Required\. Row title\. At least 1 row is required\. Supports up to 10 rows\. Maximum 24 characters\.                                                                                                                   | Priority Mail Express                                                                                                                                                                                                                          |
| <SECTION\_TITLE\_TEXT> String                  | Required\. Section title text\. At least 1 section is required\. Supports up to 10 sections\. Maximum 24 characters\.                                                                                                  | I want it ASAP\!                                                                                                                                                                                                                               |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                                                                         | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                                                                                                | \+16505551234                                                                                                                                                                                                                                  |

## Example request

Example request to send an interactive list message with a header, body, footer, and two sections containing two rows each.

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "interactive",
  "interactive": {
    "type": "list",
    "header": {
      "type": "text",
      "text": "Choose Shipping Option"
    },
    "body": {
      "text": "Which shipping option do you prefer?"
    },
    "footer": {
      "text": "Lucky Shrub: Your gateway to succulents™"
    },
    "action": {
      "button": "Shipping Options",
      "sections": [
        {
          "title": "I want it ASAP!",
          "rows": [
            {
              "id": "priority_express",
              "title": "Priority Mail Express",
              "description": "Next Day to 2 Days"
            },
            {
              "id": "priority_mail",
              "title": "Priority Mail",
              "description": "1–3 Days"
            }
          ]
        },
        {
          "title": "I can wait a bit",
          "rows": [
            {
              "id": "usps_ground_advantage",
              "title": "USPS Ground Advantage",
              "description": "2–5 Days"
            },
            {
              "id": "media_mail",
              "title": "Media Mail",
              "description": "2–8 Days"
            }
          ]
        }
      ]
    }
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```

## Webhooks

When a WhatsApp user selects an option and sends their message, a messages webhook is triggered identifying the ID (`id`) of the option they chose.

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "102290129340398",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "15550783881",
              "phone_number_id": "106540352242922"
            },
            "contacts": [
              {
                "profile": {
                  "name": "Pablo Morales"
                },
                "wa_id": "16505551234"
              }
            ],
            "messages": [
              {
                "context": {
                  "from": "15550783881",
                  "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBIwMjg0RkMxOEMyMkNEQUFFRDgA"
                },
                "from": "16505551234",
                "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgASGBQzQTZDMzFGRUFBQjlDMzIzMzlEQwA=",
                "timestamp": "1712595443",
                "type": "interactive",
                "interactive": {
                  "type": "list_reply",
                  "list_reply": {
                    "id": "priority_express",
                    "title": "Priority Mail Express",
                    "description": "Next Day to 2 Days"
                  }
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
## Example response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBJCNUQ5RUNBNTk3OEQ2M0ZEQzgA"
    }
  ]
}
```
# Interactive Location Messages

Location request messages display body text and a send location button. When a WhatsApp user taps the button, a location sharing screen appears which the user can then use to share their location. Once the user shares their location, a messages webhook is triggered, containing the user's location details.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send a location request message to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "type": "interactive",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "interactive": {
    "type": "location_request_message",
    "body": {
      "text": "<BODY_TEXT>"
    },
    "action": {
      "name": "send_location"
    }
  }
}'
```


## Request parameters

| **Placeholder**                                | **Description**                                                          | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|--------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                              | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                           | v23\.0                                                                                                                                                                                                                                         |
| <BODY\_TEXT> String                            | Required\. Message body text\. Supports URLs\. Maximum 1024 characters\. | Let's start with your pickup\. You can either manually \*enter an address\* or \*share your current location\*\.                                                                                                                               |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                           | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                  | \+16505551234                                                                                                                                                                                                                                  |

## Webhook syntax

When a WhatsApp user shares their location in response to your message, a messages webhook is triggered containing the user's location details.

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
              "display_phone_number": "<WHATSAPP_BUSINESS_DISPLAY_PHONE_NUMBER>",
              "phone_number_id": "<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>"
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
                "context": {
                  "from": "<WHATSAPP_BUSINESS_PHONE_NUMBER>",
                  "id": "<WHATSAPP_CONTEXT_MESSAGE_ID>"
                },
                "from": "<WHATSAPP_USER_ID>",
                "id": "<WHATSAPP_MESSAGE_ID>",
                "timestamp": "<TIMESTAMP>",
                "location": {
                  "address": "<LOCATION_ADDRESS>",
                  "latitude": <LOCATION_LATITUDE>,
                  "longitude": <LOCATION_LONGITUDE>,
                  "name": "<LOCATION_NAME>"
                },
                "type": "location"
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

## Webhook parameters


| **Placeholder**                                     | **Description**                                                                               | **Example Value**                                               |
|-----------------------------------------------------|-----------------------------------------------------------------------------------------------|-----------------------------------------------------------------|
| <LOCATION\_ADDRESS> String                          | Location address\. This parameter will only appear if the WhatsApp user chooses to share it\. | 1071 5th Ave, New York, NY 10128                                |
| <LOCATION\_LATITUDE> Number                         | Location latitude in decimal degrees\.                                                        | 40\.782910059774                                                |
| <LOCATION\_LONGITUDE> Number                        | Location longitude in decimal degrees\.                                                       | \-73\.959075808525                                              |
| <LOCATION\_NAME> String                             | Location name\. This parameter will only appear if the WhatsApp user chooses to share it\.    | Solomon R\. Guggenheim Museum                                   |
| <TIMESTAMP> String                                  | UNIX timestamp indicating when our servers processed the WhatsApp user's message\.            | 1702920965                                                      |
| <WHATSAPP\_BUSINESS\_ACCOUNT\_ID> String            | WhatsApp Business Account ID\.                                                                | 102290129340398                                                 |
| <WHATSAPP\_BUSINESS\_DISPLAY\_PHONE\_NUMBER> String | WhatsApp business phone number's display number\.                                             | 15550783881                                                     |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER> String          | WhatsApp business phone number\.                                                              | 15550783881                                                     |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String      | WhatsApp business phone number ID\.                                                           | 106540352242922                                                 |
| <WHATSAPP\_CONTEXT\_MESSAGE\_ID> String             | WhatsApp message ID of message that the user is responding to\.                               | wamid\.HBgLMTY0NjcwNDM1OTUVAgARGBI1QjJGRjI1RDY0RkE4Nzg4QzcA     |
| <WHATSAPP\_MESSAGE\_ID> String                      | WhatsApp message ID of the user's message\.                                                   | wamid\.HBgLMTY0NjcwNDM1OTUVAgASGBQzQTRCRDcwNzgzMTRDNTAwRTgwRQA= |
| <WHATSAPP\_USER\_ID> String                         | WhatsApp user's WhatsApp ID\.                                                                 | 16505551234                                                     |
| <WHATSAPP\_USER\_NAME> String                       | WhatsApp user's name\.                                                                        | Pablo Morales                                                   |

## Example request

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "type": "interactive",
  "to": "+16505551234",
  "interactive": {
    "type": "location_request_message",
    "body": {
      "text": "Let'\''s start with your pickup. You can either manually *enter an address* or *share your current location*."
    },
    "action": {
      "name": "send_location"
    }
  }
}'
```
## Example response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBJCNUQ5RUNBNTk3OEQ2M0ZEQzgA"
    }
  ]
}
```


# Interactive Reply Buttons Messages

Interactive reply buttons messages allow you to send up to three predefined replies for users to choose from. Users can respond to a message by selecting one of the predefined buttons, which triggers a messages webhook describing their selection.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send an interactive reply buttons message to a WhatsApp user.


```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "interactive",
  "interactive": {
    "type": "button",
    "header": {<MESSAGE_HEADER>},
    "body": {
      "text": "<BODY_TEXT>"
    },
    "footer": {
      "text": "<FOOTER_TEXT>"
    },
    "action": {
      "buttons": [
        {
          "type": "reply",
          "reply": {
            "id": "<BUTTON_ID>",
            "title": "<BUTTON_LABEL_TEXT>"
          }
        }
        <!-- Additional buttons would go here (maximum 3) -->
      ]
    }
  }
}'
```

## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                                              | **Sample Value**                                                                                                                                                                                                                                                                                                                                                                               |
|------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                                  | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD                                                                                                                                                 |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                               | v23\.0                                                                                                                                                                                                                                                                                                                                                                                         |
| <BODY\_TEXT> String                            | Required\. Body text\. URLs are automatically hyperlinked\. Maximum 1024 characters\.                                                                                        | Hi Pablo\! Your gardening workshop is scheduled for 9am tomorrow\. Use the buttons if you need to reschedule\. Thank you\!                                                                                                                                                                                                                                                                     |
| <BUTTON\_ID> String                            | Required\. A unique identifier for each button\. Supports up to 3 buttons\. Maximum 256 characters\.                                                                         | change\-button                                                                                                                                                                                                                                                                                                                                                                                 |
| <BUTTON\_LABEL\_TEXT> String                   | Required\. Button label text\. Must be unique if using multiple buttons\. Maximum 20 characters\.                                                                            | Change                                                                                                                                                                                                                                                                                                                                                                                         |
| <FOOTER\_TEXT> String                          | Required if using a footer\. Footer text\. URLs are automatically hyperlinked\. Maximum 60 characters\.                                                                      | Lucky Shrub: Your gateway to succulents\!™                                                                                                                                                                                                                                                                                                                                                     |
| <MESSAGE\_HEADER> JSON Object                  | Optional\. Header content\. Supports the following types: document image text video Media assets can be sent using their uploaded media id or URL link \(not recommended\)\. | Image header example using uploaded media ID \(same basic structure for all media types\): \{   "type": "image",   "image": \{     "id": "2762702990552401" \} Image header example using hosted media: \{   "type": "image",   "image": \{     "link": "https://www\.luckyshrub\.com/media/workshop\-banner\.png" \} Text header example: \{   "type":"text",   "text": "Workshop Details" \} |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                               | 106540352242922                                                                                                                                                                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                                                      | \+16505551234                                                                                                                                                                                                                                                                                                                                                                                  |

## Example request

Example request to send an interactive reply buttons message with an image header, body text, footer text, and two quick-reply buttons.


```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "interactive",
  "interactive": {
    "type": "button",
    "header": {
      "type": "image",
      "image": {
        "id": "2762702990552401"
      }
    },
    "body": {
      "text": "Hi Pablo! Your gardening workshop is scheduled for 9am tomorrow. Use the buttons if you need to reschedule. Thank you!"
    },
    "footer": {
      "text": "Lucky Shrub: Your gateway to succulents!™"
    },
    "action": {
      "buttons": [
        {
          "type": "reply",
          "reply": {
            "id": "change-button",
            "title": "Change"
          }
        },
        {
          "type": "reply",
          "reply": {
            "id": "cancel-button",
            "title": "Cancel"
          }
        }
      ]
    }
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```

## Webhooks

When a WhatsApp user taps on a reply button, a messages webhook is triggered that describes their selection in a `button_reply` object:

```json
"button_reply": {
  "id": "<BUTTON_ID>",
  "title": "<BUTTON_LABEL_TEXT>"
}  
```

- `<BUTTON_ID>` — The button ID of the button tapped by the user.
- `<BUTTON_LABEL_TEXT>` — The button label text of the button tapped by the user.

## Example Webhook

```json
{
  "object": "whatsapp_business_account",
  "entry": [
    {
      "id": "102290129340398",
      "changes": [
        {
          "value": {
            "messaging_product": "whatsapp",
            "metadata": {
              "display_phone_number": "15550783881",
              "phone_number_id": "106540352242922"
            },
            "contacts": [
              {
                "profile": {
                  "name": "Pablo Morales"
                },
                "wa_id": "16505551234"
              }
            ],
            "messages": [
              {
                "context": {
                  "from": "15550783881",
                  "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBJBM0Y4RUU0RUNFQkFDMjYzQUMA"
                },
                "from": "16505551234",
                "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgASGBQzQThBREYwNzc2RDc2QjA1QTIwMgA=",
                "timestamp": "1714510003",
                "type": "interactive",
                "interactive": {
                  "type": "button_reply",
                  "button_reply": {
                    "id": "change-button",
                    "title": "Change"
                  }
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
# Location Messages

Location messages allow you to send a location's latitude and longitude coordinates to a WhatsApp user.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send a location message to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "location",
  "location": {
    "latitude": "<LOCATION_LATITUDE>",
    "longitude": "<LOCATION_LONGITUDE>",
    "name": "<LOCATION_NAME>",
    "address": "<LOCATION_ADDRESS>"
  }
}'
```

## Request parameters

| **Placeholder**                                | **Description**                                    | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|----------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.        | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                     | v23\.0                                                                                                                                                                                                                                         |
| <LOCATION\_ADDRESS> String                     | Optional\. Location address\.                      | 101 Forest Ave, Palo Alto, CA 94301                                                                                                                                                                                                            |
| <LOCATION\_LATITUDE> String                    | Required\. Location latitude in decimal degrees\.  | 37\.4421625186868                                                                                                                                                                                                                              |
| <LOCATION\_LONGITUDE> String                   | Required\. Location longitude in decimal degrees\. | \-122\.161535820494                                                                                                                                                                                                                            |
| <LOCATION\_NAME> String                        | Optional\. Location name\.                         | Philz Coffee                                                                                                                                                                                                                                   |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.     | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.            | \+16505551234                                                                                                                                                                                                                                  |

## Example request

Example request to send a location message with a name and address.

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "location",
  "location": {
    "latitude": "37.44216251868683",
    "longitude": "-122.16153582049394",
    "name": "Philz Coffee",
    "address": "101 Forest Ave, Palo Alto, CA 94301"
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Text Messages

Text messages are messages containing only a text body and an optional link preview.


## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages` endpoint to send a text message to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "text",
  "text": {
    "preview_url": <ENABLE_LINK_PREVIEW>,
    "body": "<BODY_TEXT>"
  }
}
```

## Request parameters

| **Placeholder**                                | **Description**                                                                                                                                   | **Example Value**                                                                                                                                                                                                                              |
|------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                       | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                    | v23\.0                                                                                                                                                                                                                                         |
| <BODY\_TEXT> String                            | Required\. Body text\. URLs are automatically hyperlinked\. Maximum 1024 characters\.                                                             | As requested, here's the link to our latest product: https://www\.meta\.com/quest/quest\-3/                                                                                                                                                    |
| <ENABLE\_LINK\_PREVIEW> Boolean                | Optional\. Set to true to have the WhatsApp client attempt to render a link preview of any URL in the body text string\. See Link Preview below\. | true                                                                                                                                                                                                                                           |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                    | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String         | Required\. WhatsApp user phone number\.                                                                                                           | \+16505551234                                                                                                                                                                                                                                  |

## Link preview

You can have the WhatsApp client attempt to render a preview of the first URL in the body text string, if it contains one. URLs must begin with `http://` or `https://`. If multiple URLs are in the body text string, only the first URL will be rendered.

If omitted, or if unable to retrieve a link preview, a clickable link will be rendered instead.

## Example request

Example request to send a text message with link previews enabled and a body text string that contains a link.

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "text",
  "text": {
    "preview_url": true,
    "body": "As requested, here'\''s the link to our latest product: https://www.meta.com/quest/quest-3/"
  }
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
# Video Messages

Video messages display a thumbnail preview of a video image with an optional caption. When the WhatsApp user taps the preview, it loads the video and displays it to the user.

## Request syntax

Use the `POST /<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages ` endpoint to send a video message to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/<API_VERSION>/<WHATSAPP_BUSINESS_PHONE_NUMBER_ID>/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer <ACCESS_TOKEN>' \
-d '
{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "<WHATSAPP_USER_PHONE_NUMBER>",
  "type": "video",
  "video": {
    "id" : "<MEDIA_ID>", /* Only if using uploaded media */
    "link": "<MEDIA_URL>", /* Only if linking to your media */
    "caption": "<VIDEO_CAPTION_TEXT>"
  }
}
'
```

## Request parameters


| **Placeholder**                        | **Description**                                                                                                                                                                        | **Example Value**                                                       |
|----------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------|
| <ACCESS\_TOKEN> String                         | Required\. System token or business token\.                                                                                                                                                   | EAAAN6tcBzAUBOZC82CW7iR2LiaZBwUHS4Y7FDtQxRUPy1PHZClDGZBZCgWdrTisgMjpFKiZAi1FBBQNO2IqZBAzdZAA16lmUs0XgRcCf6z1LLxQCgLXDEpg80d41UZBt1FKJZCqJFcTYXJvSMeHLvOdZwFyZBrV9ZPHZASSqxDZBUZASyFdzjiy2A1sippEsF4DVV5W2IlkOSr2LrMLuYoNMYBy8xQczzOKDOMccqHEZD |
| <API\_VERSION> String                          | Optional\. Graph API version\.                                                                                                                                                                | v23\.0                                                                                                                                                                                                                                         |
| <VIDEO\_CAPTION\_TEXT> String          | Optional\. Video caption text\. Maximum 1024 characters\.                                                                                                                              | A succulent eclipse\!                                                   |
| <MEDIA\_ID> String                     | Required if using an uploaded media asset \(recommended\)\. Uploaded media asset ID\.                                                                                                  | 1166846181421424                                                        |
| <MEDIA\_URL> String                    | Required if linking to your media asset \(not recommended\) URL of video asset on your public server\. For better performance, we recommend that you upload your media asset instead\. | https://www\.luckyshrub\.com/assets/lucky\-shrub\-eclipse\-viewing\.mp4 |
| <WHATSAPP\_BUSINESS\_PHONE\_NUMBER\_ID> String | Required\. WhatsApp business phone number ID\.                                                                                                                                                | 106540352242922                                                                                                                                                                                                                                |
| <WHATSAPP\_USER\_PHONE\_NUMBER> String | Required\. WhatsApp user phone number\.                                                                                                                                                | \+16505551234                                                           |

## Supported document formats

Only H.264 video codec and AAC audio codec supported. Single audio stream or no audio stream only.

| **Video Type** | **Extension** | **MIME Type** | **Max Size** |
|----------------|---------------|---------------|--------------|
| 3GPP           | .3gp          | video/3gpp    | 16 MB        |
| MP4 Video      | .mp4          | video/mp4     | 16 MB        |

## Example request

Example request to send a video message with a caption to a WhatsApp user.

```bash
curl 'https://graph.facebook.com/v23.0/106540352242922/messages' \
-H 'Content-Type: application/json' \
-H 'Authorization: Bearer EAAJB...' \
-d '{
  "messaging_product": "whatsapp",
  "recipient_type": "individual",
  "to": "+16505551234",
  "type": "video",
  "video": {
    "id" : "1166846181421424",
    "caption": "A succulent eclipse!"
  }
}'
```

## Example Response

```json
{
  "messaging_product": "whatsapp",
  "contacts": [
    {
      "input": "+16505551234",
      "wa_id": "16505551234"
    }
  ],
  "messages": [
    {
      "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
    }
  ]
}
```
