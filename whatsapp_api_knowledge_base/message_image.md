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
