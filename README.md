# Photos.network

[![License](https://img.shields.io/github/license/photos-network/core?style=for-the-badge)](./LICENSE.md)
[![GitHub contributors](https://img.shields.io/github/contributors/photos-network/core?color=success&style=for-the-badge)](https://github.com/photos-network/core/graphs/contributors)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/photos-network/core/check.yaml?style=for-the-badge)


[Photos.network](https://photos.network) is a free and open source, privacy first, self-hosted photo storage and sharing service for fediverse.

Its core features are:

- Share photos with friends, family or public
- Filter / Search photos by attributes like location or date
- Group photos by their content like people or objects
- Upload photos and videos without resolution or quality constraints



## 🧠 Core

This repository contains the **core** system of the project.
It is responsible for main tasks e.g.:

- **Authentication** (validate the identity of users)
- **Authorization** (handle access privileges of resources like photos or albums)
- **Plugin Handling** (extend the feature-set by plugins)
- **Persistency** (read / write data)
- **Task Processing** (keep track of running tasks)

## 🛠️ Setup

To have a proper experience, add those to your nginx config.
```conf
location ~* /albums/.*/download {
    proxy_buffering off;
    proxy_read_timeout 300s;
    proxy_set_header Upgrade $http_upgrade;
    proxy_set_header Connection $http_connection;
    proxy_http_version 1.1;
    include conf.d/include/proxy.conf;
}
```



## 🧪 Development

The core is written in 🦀 [Rust](https://rust-lang.org/) and highly customizably by using a Plugin-system.



#### 🔬 Testing

To run tests for all crates in this workspace, run:
```shell
$ cargo test --workspace --all-targets
```

Run continuous checks on a specific crate during development.

```shell
$ cargo watch --exec 'test -p media'
```



## 📜 Roadmap (MvP)

 - Create a new media item
 - Upload one or multiple images
 - Download a list of owned media items
 - Download the original file for a specific media item
 - Authenticate via openID



## 🧩 Contribution

This is a free and open project and lives from contributions of the community.

See our [Contribution Guide](CONTRIBUTING.md)



## 🏛️ License

```
Photos.network · A privacy first photo storage and sharing service for fediverse
Copyright (C) 2020 Photos network developers

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
