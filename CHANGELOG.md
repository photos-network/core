## [0.5.1] - 2022-07-07
### Removed
- unused client_secret from oauth calls


## [0.5.0] - 2022-07-07
### Added
- generate random client credentials to fallback configuration

### Changed
- renamed configuration file to `core_configuration.json`


## [0.4.0] - 2021-12-11
### Added
- system port to image response

### Changed
- return with `HTTPUnauthorized` instead of `HTTPForbidden`


## [0.3.0] - 2021-10-03
### Added
- Enrich photo list metadata

### Changed
- changed request url to `api` instead of `v1`


## [0.2.1] - 2020-05-09
### Changed
- trim and lowercase username on oauth login


## [0.2.0] - 2020-05-06
### Added
- dynamic client loading from `configuration.json`
- simplified user management


## [0.0.1] - 2021-03-11
### Added
- integrated oauth authorization server
- dynamic addon loading with dedicated setup step
- async file logging

[0.5.1]: https://github.com/photos-network/core/compare/Release/v0.5.0...Release/v0.5.1
[0.5.0]: https://github.com/photos-network/core/compare/Release/v0.4.0...Release/v0.5.0
[0.4.0]: https://github.com/photos-network/core/compare/Release/v0.3.0...Release/v0.4.0
[0.3.0]: https://github.com/photos-network/core/compare/Release/v0.2.1...Release/v0.3.0
[0.2.1]: https://github.com/photos-network/core/compare/Release/v0.2.0...Release/v0.2.1
[0.2.0]: https://github.com/photos-network/core/compare/Release/v0.0.1...Release/v0.2.0
[0.0.1]: https://github.com/photos-network/core/releases/tag/Release/v0.0.1
