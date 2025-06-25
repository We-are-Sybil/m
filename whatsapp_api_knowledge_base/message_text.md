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
