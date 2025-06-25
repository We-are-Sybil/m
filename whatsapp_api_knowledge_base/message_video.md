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
