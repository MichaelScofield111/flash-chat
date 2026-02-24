POST http://localhost:6688/api/signup
{
    "fullname": "Alice Chen",
    "email": "alice@acme.org",
    "password": "123456"
}
HTTP *
[Asserts]
status >= 200
status <= 409

POST http://localhost:6688/api/signin
Content-Type: application/json
{
    "email": "alice@acme.org",
    "password": "123456"
}
HTTP 200
[Captures]
token: jsonpath "$.token"

GET http://localhost:6688/api/chat
Authorization: Bearer {{token}}
HTTP 200
[Asserts]
body == "chat"
