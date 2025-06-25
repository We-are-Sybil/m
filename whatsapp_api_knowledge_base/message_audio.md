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
