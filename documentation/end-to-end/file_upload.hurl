# create a new media item
POST http://127.0.0.1:7777/media
Authorization: FakeToken

[MultipartFormData]
name: DSC_1234
date_taken: 1985-04-12T23:20:50.52Z

HTTP 201

[Asserts]
jsonpath "$.id" != null

[Captures]
media_id: jsonpath "$.id"


# upload reference to media item
POST http://127.0.0.1:7777/media/{{media_id}}
Authorization: FakeToken

[MultipartFormData]
name: DSC_1234.NEF
file: file,DSC_1234.NEF;

HTTP 201


# check media item
GET http://127.0.0.1:7777/media/{{media_id}}
Authorization: FakeToken

HTTP 200
[Asserts]
jsonpath "$.{{media_id}}.references" count == 1
