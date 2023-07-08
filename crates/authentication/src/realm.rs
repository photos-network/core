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

use openidconnect::core::{
    CoreClaimName, CoreJsonWebKeySet, CoreJwsSigningAlgorithm, CoreProviderMetadata,
    CoreResponseType, CoreRsaPrivateSigningKey, CoreSubjectIdentifierType,
};
use openidconnect::{
    AuthUrl, EmptyAdditionalProviderMetadata, IssuerUrl, JsonWebKeyId, JsonWebKeySetUrl,
    PrivateSigningKey, ResponseTypes, TokenUrl, UserInfoUrl,
};

use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::client::Client;
use crate::error::Error;
use crate::request::AuthRequest;

#[derive(Debug, Clone)]
pub struct Realm {
    #[allow(dead_code)]
    pub(crate) name: String,
    pub(crate) clients: Vec<Client>,
    pub(crate) domain: String,
    pub(crate) provider_metadata: CoreProviderMetadata,
    pub(crate) jwks: CoreJsonWebKeySet,
    pub(crate) requests: Vec<AuthRequest>,
}

impl Realm {
    pub fn new<P: AsRef<Path>>(
        name: &str,
        domain: &str,
        scheme: &str,
        clients: Vec<Client>,
        realm_keys_base_path: P,
    ) -> Result<Self, Error> {
        let mut realm_key_file = File::open(
            realm_keys_base_path
                .as_ref()
                .join(name)
                .with_extension("pem"),
        ).expect(&format!("key ({}) not found in directory ({})!", name, realm_keys_base_path.as_ref().display()));
        let mut realm_key_str = String::new();
        realm_key_file
            .read_to_string(&mut realm_key_str)
            .map_err(|_| Error::CouldNotOpenRealmKey(name.to_owned()))?;

        Ok(Self {
            name: name.to_owned(),
            domain: domain.to_owned(),
            clients,
            requests: vec![],
            provider_metadata: CoreProviderMetadata::new(
                // Parameters required by the OpenID Connect Discovery spec.
                IssuerUrl::new(format!("{}://{}", scheme, domain))?,
                AuthUrl::new(format!("{}://{}/oidc/authorize", scheme, domain))?,
                // Use the JsonWebKeySet struct to serve the JWK Set at this URL.
                JsonWebKeySetUrl::new(format!("{}://{}/oidc/jwk", scheme, domain))?,
                // Supported response types (flows).
                vec![
                    // Recommended: support the code flow.
                    ResponseTypes::new(vec![CoreResponseType::Code]),
                ],
                // For user privacy, the Pairwise subject identifier type is preferred. This prevents
                // distinct relying parties (clients) from knowing whether their users represent the same
                // real identities. This identifier type is only useful for relying parties that don't
                // receive the 'email', 'profile' or other personally-identifying scopes.
                // The Public subject identifier type is also supported.
                vec![CoreSubjectIdentifierType::Pairwise],
                // Support the RS256 signature algorithm.
                vec![CoreJwsSigningAlgorithm::RsaSsaPssSha256],
                // OpenID Connect Providers may supply custom metadata by providing a struct that
                // implements the AdditionalProviderMetadata trait. This requires manually using the
                // generic ProviderMetadata struct rather than the CoreProviderMetadata type alias,
                // however.
                EmptyAdditionalProviderMetadata {},
            )
            // Specify the token endpoint (required for the code flow).
            .set_token_endpoint(Some(TokenUrl::new(format!(
                "{}://{}/oidc/token",
                scheme, domain
            ))?))
            // Recommended: support the UserInfo endpoint.
            .set_userinfo_endpoint(Some(UserInfoUrl::new(format!(
                "{}://{}/oidc/userinfo",
                scheme, domain
            ))?))
            // Recommended: specify the supported scopes.
            .set_scopes_supported(Some(vec![
                openidconnect::Scope::new("openid".to_string()),
                openidconnect::Scope::new("email".to_string()),
                openidconnect::Scope::new("profile".to_string()),
                openidconnect::Scope::new("library.read".to_string()),
                openidconnect::Scope::new("library.append".to_string()),
                openidconnect::Scope::new("library.edit".to_string()),
                openidconnect::Scope::new("library.write".to_string()),
                openidconnect::Scope::new("library.share".to_string()),
                openidconnect::Scope::new("admin.users:read".to_string()),
                openidconnect::Scope::new("admin.users:invite".to_string()),
                openidconnect::Scope::new("admin.users:write".to_string()),
            ]))
            // Recommended: specify the supported ID token claims.
            .set_claims_supported(Some(vec![
                // Providers may also define an enum instead of using CoreClaimName.
                CoreClaimName::new("sub".to_string()),
                CoreClaimName::new("aud".to_string()),
                CoreClaimName::new("email".to_string()),
                CoreClaimName::new("email_verified".to_string()),
                CoreClaimName::new("exp".to_string()),
                CoreClaimName::new("iat".to_string()),
                CoreClaimName::new("iss".to_string()),
                CoreClaimName::new("name".to_string()),
                CoreClaimName::new("given_name".to_string()),
                CoreClaimName::new("family_name".to_string()),
                CoreClaimName::new("picture".to_string()),
                CoreClaimName::new("locale".to_string()),
            ])),
            jwks: CoreJsonWebKeySet::new(vec![
                // RSA keys may also be constructed directly using CoreJsonWebKey::new_rsa(). Providers
                // aiming to support other key types may provide their own implementation of the
                // JsonWebKey trait or submit a PR to add the desired support to this crate.
                CoreRsaPrivateSigningKey::from_pem(
                    &realm_key_str,
                    Some(JsonWebKeyId::new(format!("{}_key", name))),
                )
                .expect("Invalid RSA private key")
                .as_verification_key(),
            ]),
        })
    }
}
