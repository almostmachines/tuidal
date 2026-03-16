## Core

### Data Structures

**User**

User {
    country*: String (two characters)
    email: String
    email_verified: Boolean
    first_name: String
    id*: String
    authenticated: Boolean
    last_name: String
    nostr_public_key: String
    username*: String
}

## Auth

### Data Structures

**AccessToken**

- token string, expiry (expires_in), and granted scopes

**RefreshToken**

- newtype around the token string

**Scope**

- enum of possible scopes (the docs reference them but don't enumerate; you'd populate as you discover them)

**ClientCredentials**

- client_id + client_secret

PKCE types (required by the authorization code flow):

**CodeVerifier**

- the cryptographically random key

**CodeChallenge**

- the S256-transformed value derived from the verifier

#### Flow-related types:

**AuthorizationCode**

- newtype for the code received from the authorize redirect

**RedirectUri**

- newtype for the configured redirect URI

**State**

- the optional CSRF protection parameter

#### Session/state:

**AuthSession**

- holds the current AccessToken + optional RefreshToken, representing the authenticated state your app works with
