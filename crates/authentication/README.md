# Authentication

This crate provides an **Authorization code flow with PKCE** for [Photos.network](https://photos.network).

Public clients (e.g. native and single-page applications) cannot securely store client secrets. 

### Native apps
Decompiling the app will reveal the Client Secret, which is bound to the app and is the same for all users and devices. Also they make use of a custom URL scheme to capture redirects (e.g., photosapp://) potentially allowing malicious applications to receive an Authorization Code from your Authorization Server.

### Single-page application
Cannot securely store a Client Secret because their entire source is available to the browser.


With [PKCE](https://datatracker.ietf.org/doc/html/rfc7636) an application created pair of secrets (Code Verifier & Code Challenge) is send to the **authorization server** over HTTPS. This way a malicious attacker can only intercept the Authorization Code but can't exchange it for a token without the Code Verifier.

## Authorization code flow with PKCE
```mermaid
sequenceDiagram;
    participant U as Users
    participant A as App
    participant AM as Authorization Server
    participant R as Resource Server
    
    U->>A: User clicks login
    A->>AM: GET http://127.0.0.1/.well-known/openid-configuration

    AM->>A: JSON meta-data document
    Note right of A: { "authorization_endpoint":"http://localhost:7777/oidc/authorize", <br> "token_endpoint":"http://localhost:7777/oidc/token" ...
    
    A->>A: Generate Code Verifier & Challenge
    
    A->>AM: GET http://localhost:7777/oidc/authorize?[...]
    Note right of A: GET /oidc/authorize parameters: <br> response_type=id_token%20token <br> client_id=mobile-app (identifier) <br> redirect_uri=photosapp://authenticate <br> state=xxxx (CRFS protection) <br> nonce=xyz (server-side replay protection) <br> scope=openid email profile library:read <br> code_challenge=elU6u5zyqQT2f92GRQUq6PautAeNDf4DQPayy <br> code_challenge_method=S256
    
    AM->>U: show login prompt
    U->>AM: perform login and grant consent
    AM->>A: 302 Redirect to http://127.0.0.1/callback?[...] (redirect_uri)
    Note right of A: GET /callback <br> state=xxx <br> code=xxx
    
    A->>AM: GET http://localhost:7777/oidc/token
    Note right of A: GET /oidc/token parameters: <br> client_id=xxx (identifier) <br> redirect_uri=http://127.0.0.1/callback <br> code_verifier=xxxx (generated verifier) <br> code=xyz (authorization_code) <br> grant_type=authorization_code
    AM->>AM: Validate code verifier and challenge
    AM->>A: ID Token and Access token
    Note right of A: { <br> "token_type": "Bearer", <br> "expires_in":   3600, <br> "access_token": "eyJraWQiOiI3bFV0aGJyR2hWVmx...",<br> "id_token":     "eyJraWQiOiI3bFV0aGJyR2hWVmx...",<br> "scope":        "profile openid email" <br> }

    %% R->>A: return user attributes
    %% U->>O: GET http://127.0.0.1/callback?[...]
    %% O->>A: POST http://127.0.0.1:7777/oidc/token
    %% Note right of O: POST /oidc/token <br> client_id=xxx <br> grant_type=authorization_code <br> code=xxx <br> state=xxx
    %% A->>O: JSON { "base64(id_token)", access_token" }
    %% O->>O: verify id_token signature is valid and signed
    %% O->>U: GET 302 Redirect to http://127.0.0.1
    Note over A: User is authenticate to http://127.0.0.1
    A->>R: Request user data with Access Token
    R->>A: responds with requested data
```
