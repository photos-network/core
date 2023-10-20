# Chain HTTP requests with curl and capture and verify the results.

# API up and running
GET http://127.0.0.1:7777

HTTP 200
[Asserts]
jsonpath "$.message" == "API running"


# OIDC discovery
GET http://127.0.0.1:7777/.well-known/openid-configuration

HTTP 200
[Asserts]
jsonpath "$.issuer" == "http://localhost:7777"
jsonpath "$.authorization_endpoint" == "http://localhost:7777/oidc/authorize"
jsonpath "$.token_endpoint" == "http://localhost:7777/oidc/token"
jsonpath "$.userinfo_endpoint" == "http://localhost:7777/oidc/userinfo"
jsonpath "$.jwks_uri" == "http://localhost:7777/oidc/jwk"
jsonpath "$.scopes_supported" count == 11
jsonpath "$.scopes_supported" includes "email"
jsonpath "$.scopes_supported" includes "library.read"
jsonpath "$.scopes_supported" includes "library.append"
jsonpath "$.scopes_supported" includes "library.edit"
jsonpath "$.scopes_supported" includes "library.write"
jsonpath "$.scopes_supported" includes "library.share"

# OIDC authorization flow with PKCE
GET http://127.0.0.1:7777/oidc/authorize?response_type=id_token%20token&client_id=mobile-app&state=12345&code_challenge=47DEQpj8HBSa-_TImW-5JCeuQeRkm5NMpJWZG3hSuFU&code_challenge_method=S256&redirect_uri=photosapp://authenticate&scope=openid%20profile%20email%20phone%20library:read&nonce=ABCDE

HTTP 303
[Asserts]
header "Location" exists
header "Location" contains "login"
header "Location" == "/master/login"


GET http://localhost:7777/master/login
HTTP 200



POST http://localhost:7777/master/login
[FormParams]
username: toto
password: 12345678

HTTP 200
