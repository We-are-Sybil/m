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
