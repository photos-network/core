/* Photos.network · A privacy first photo storage and sharing service for fediverse.
 * Copyright (C) 2020 Photos network developers
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! This crate offers an **Authorization code flow with PKCE** in a [Photos.network](https://photos.network) core application.
//! 
//! To identify users and granting them access to the applications content, the Open Authorization (OAuth) standard is used so users can login without sharing credentials theirselfs.
//!
use axum::Router;
use anyhow::{anyhow, Ok};
use openidconnect::core::{
    CoreAuthenticationFlow, CoreClient, CoreProviderMetadata, CoreResponseType, CoreUserInfoClaims,
};
use openidconnect::{
    AccessTokenHash, AuthorizationCode, ClientId, ClientSecret, CsrfToken,
    EmptyAdditionalProviderMetadata, IssuerUrl, Nonce, PkceCodeChallenge, PkceCodeVerifier,
    ProviderMetadata, RedirectUrl, Scope,
};

use anyhow::Result;
use openidconnect::reqwest::http_client;
use thiserror::Error;

// Use OpenID Connect Discovery to fetch the provider metadata.
use openidconnect::{OAuth2TokenResponse, TokenResponse};

pub struct AuthenticationManager {
    pub client: openidconnect::Client<
        openidconnect::EmptyAdditionalClaims,
        openidconnect::core::CoreAuthDisplay,
        openidconnect::core::CoreGenderClaim,
        openidconnect::core::CoreJweContentEncryptionAlgorithm,
        openidconnect::core::CoreJwsSigningAlgorithm,
        openidconnect::core::CoreJsonWebKeyType,
        openidconnect::core::CoreJsonWebKeyUse,
        openidconnect::core::CoreJsonWebKey,
        openidconnect::core::CoreAuthPrompt,
        openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
        openidconnect::StandardTokenResponse<
            openidconnect::IdTokenFields<
                openidconnect::EmptyAdditionalClaims,
                openidconnect::EmptyExtraTokenFields,
                openidconnect::core::CoreGenderClaim,
                openidconnect::core::CoreJweContentEncryptionAlgorithm,
                openidconnect::core::CoreJwsSigningAlgorithm,
                openidconnect::core::CoreJsonWebKeyType,
            >,
            openidconnect::core::CoreTokenType,
        >,
        openidconnect::core::CoreTokenType,
        openidconnect::StandardTokenIntrospectionResponse<
            openidconnect::EmptyExtraTokenFields,
            openidconnect::core::CoreTokenType,
        >,
        openidconnect::core::CoreRevocableToken,
        openidconnect::StandardErrorResponse<openidconnect::RevocationErrorResponseType>,
    >,
    pub provider_metadata: ProviderMetadata<
        EmptyAdditionalProviderMetadata,
        openidconnect::core::CoreAuthDisplay,
        openidconnect::core::CoreClientAuthMethod,
        openidconnect::core::CoreClaimName,
        openidconnect::core::CoreClaimType,
        openidconnect::core::CoreGrantType,
        openidconnect::core::CoreJweContentEncryptionAlgorithm,
        openidconnect::core::CoreJweKeyManagementAlgorithm,
        openidconnect::core::CoreJwsSigningAlgorithm,
        openidconnect::core::CoreJsonWebKeyType,
        openidconnect::core::CoreJsonWebKeyUse,
        openidconnect::core::CoreJsonWebKey,
        openidconnect::core::CoreResponseMode,
        CoreResponseType,
        openidconnect::core::CoreSubjectIdentifierType,
    >,
    pub pkce_challenge: PkceCodeChallenge,
    pub pkce_verifier: PkceCodeVerifier,
}

impl AuthenticationManager {
    pub fn routes<S>() -> Router<S>
    where
        S: Send + Sync + 'static + Clone,
    {
        Router::new()
            .layer(tower_http::trace::TraceLayer::new_for_http())
    }
}

#[derive(Debug, Error)]
enum AuthError {
}

impl AuthenticationManager {
    pub fn new() -> Result<Self, anyhow::Error> {
        tracing::error!("run setup");

        let foo = CoreProviderMetadata::discover(
            &IssuerUrl::new("https://accounts.google.com".to_string())?,
            http_client,
        )?;

        // Generate a PKCE challenge.
        let (pkce_challenge, pkce_verifier): (PkceCodeChallenge, openidconnect::PkceCodeVerifier) =
            PkceCodeChallenge::new_random_sha256();

        Ok(Self {
            provider_metadata: foo.clone(),
            client: CoreClient::from_provider_metadata(
                foo,
                ClientId::new(
                    "953760225864-77il4losuech1dtsea36tmma2e8bko3h.apps.googleusercontent.com"
                        .to_string(),
                ),
                Some(ClientSecret::new(
                    "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".to_string(),
                )),
            )
            // Set the URL the user will be redirected to after the authorization process.
            .set_redirect_uri(RedirectUrl::new("http://127.0.0.1/callback".to_string())?),
            pkce_challenge,
            pkce_verifier,
        })
    }

    pub fn create_authorization_url(
        client: openidconnect::Client<
            openidconnect::EmptyAdditionalClaims,
            openidconnect::core::CoreAuthDisplay,
            openidconnect::core::CoreGenderClaim,
            openidconnect::core::CoreJweContentEncryptionAlgorithm,
            openidconnect::core::CoreJwsSigningAlgorithm,
            openidconnect::core::CoreJsonWebKeyType,
            openidconnect::core::CoreJsonWebKeyUse,
            openidconnect::core::CoreJsonWebKey,
            openidconnect::core::CoreAuthPrompt,
            openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
            openidconnect::StandardTokenResponse<
                openidconnect::IdTokenFields<
                    openidconnect::EmptyAdditionalClaims,
                    openidconnect::EmptyExtraTokenFields,
                    openidconnect::core::CoreGenderClaim,
                    openidconnect::core::CoreJweContentEncryptionAlgorithm,
                    openidconnect::core::CoreJwsSigningAlgorithm,
                    openidconnect::core::CoreJsonWebKeyType,
                >,
                openidconnect::core::CoreTokenType,
            >,
            openidconnect::core::CoreTokenType,
            openidconnect::StandardTokenIntrospectionResponse<
                openidconnect::EmptyExtraTokenFields,
                openidconnect::core::CoreTokenType,
            >,
            openidconnect::core::CoreRevocableToken,
            openidconnect::StandardErrorResponse<openidconnect::RevocationErrorResponseType>,
        >,
        pkce_challenge: PkceCodeChallenge,
    ) -> Result<Nonce> {
        let nonce = Nonce::new_random;
        // Generate the full authorization URL.
        let (auth_url, _csrf_token, nonce) = client
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                nonce,
            )
            // Set the desired scopes.
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            // Set the PKCE code challenge.
            .set_pkce_challenge(pkce_challenge)
            .url();

        // This is the URL you should redirect the user to, in order to trigger the authorization
        // process.
        println!("Browse to: {}", auth_url);

        Ok(nonce)
    }

    pub fn exchange_code(
        client: openidconnect::Client<
            openidconnect::EmptyAdditionalClaims,
            openidconnect::core::CoreAuthDisplay,
            openidconnect::core::CoreGenderClaim,
            openidconnect::core::CoreJweContentEncryptionAlgorithm,
            openidconnect::core::CoreJwsSigningAlgorithm,
            openidconnect::core::CoreJsonWebKeyType,
            openidconnect::core::CoreJsonWebKeyUse,
            openidconnect::core::CoreJsonWebKey,
            openidconnect::core::CoreAuthPrompt,
            openidconnect::StandardErrorResponse<openidconnect::core::CoreErrorResponseType>,
            openidconnect::StandardTokenResponse<
                openidconnect::IdTokenFields<
                    openidconnect::EmptyAdditionalClaims,
                    openidconnect::EmptyExtraTokenFields,
                    openidconnect::core::CoreGenderClaim,
                    openidconnect::core::CoreJweContentEncryptionAlgorithm,
                    openidconnect::core::CoreJwsSigningAlgorithm,
                    openidconnect::core::CoreJsonWebKeyType,
                >,
                openidconnect::core::CoreTokenType,
            >,
            openidconnect::core::CoreTokenType,
            openidconnect::StandardTokenIntrospectionResponse<
                openidconnect::EmptyExtraTokenFields,
                openidconnect::core::CoreTokenType,
            >,
            openidconnect::core::CoreRevocableToken,
            openidconnect::StandardErrorResponse<openidconnect::RevocationErrorResponseType>,
        >,
        pkce_verifier: PkceCodeVerifier,
        authorization_code: String,
        nonce: Nonce,
    ) -> Result<()> {
        // Once the user has been redirected to the redirect URL, you'll have access to the
        // authorization code. For security reasons, your code should verify that the `state`
        // parameter returned by the server matches `csrf_state`.

        // Now you can exchange it for an access token and ID token.
        let token_response = client
            .exchange_code(AuthorizationCode::new(authorization_code))
            // Set the PKCE code verifier.
            .set_pkce_verifier(pkce_verifier)
            .request(http_client)?;

        // Extract the ID token claims after verifying its authenticity and nonce.
        let id_token = token_response
            .id_token()
            .ok_or_else(|| anyhow!("Server did not return an ID token"))?;
        let claims = id_token.claims(&client.id_token_verifier(), &nonce)?;

        // Verify the access token hash to ensure that the access token hasn't been substituted for
        // another user's.
        if let Some(expected_access_token_hash) = claims.access_token_hash() {
            let actual_access_token_hash = AccessTokenHash::from_token(
                token_response.access_token(),
                &id_token.signing_alg()?,
            )?;
            if actual_access_token_hash != *expected_access_token_hash {
                return Err(anyhow!("Invalid access token"));
            }
        }

        // The authenticated user's identity is now available. See the IdTokenClaims struct for a
        // complete listing of the available claims.
        println!(
            "User {} with e-mail address {} has authenticated successfully",
            claims.subject().as_str(),
            claims
                .email()
                .map(|email| email.as_str())
                .unwrap_or("<not provided>"),
        );

        // If available, we can use the UserInfo endpoint to request additional information.

        // The user_info request uses the AccessToken returned in the token response. To parse custom
        // claims, use UserInfoClaims directly (with the desired type parameters) rather than using the
        // CoreUserInfoClaims type alias.
        let _userinfo: CoreUserInfoClaims = client
            .user_info(token_response.access_token().to_owned(), None)
            .map_err(|err| anyhow!("No user info endpoint: {:?}", err))?
            .request(http_client)
            .map_err(|err| anyhow!("Failed requesting user info: {:?}", err))?;

        // See the OAuth2TokenResponse trait for a listing of other available fields such as
        // access_token() and refresh_token().

        Ok(())
    }
}
