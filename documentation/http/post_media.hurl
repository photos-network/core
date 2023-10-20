POST http://127.0.0.1:7777/media
Authorization: FakeToken
Connection: Keep-Alive

[MultipartFormData]
name: DSC_1234
date_taken: 1985-04-12T23:20:50.52Z

HTTP 200

[Asserts]
jsonpath "$.id" != null
