# Photos.network

[![License](https://img.shields.io/github/license/photos-network/core?style=for-the-badge)](./LICENSE.md)
[![GitHub contributors](https://img.shields.io/github/contributors/photos-network/core?color=success&style=for-the-badge)](https://github.com/photos-network/core/graphs/contributors)
[![Discord](https://img.shields.io/discord/793235453871390720?style=for-the-badge)](https://discord.gg/dGFDpmWp46)
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



## 🧩 Contribution

This is a free and open project and lives from contributions of the community.

See our [Contribution Guide](CONTRIBUTING.md)



## 🧪 Development

The core is written in 🦀 [Rust](https://rust-lang.org/) and highly customizably by using a Plugin-system.



#### 📄 Documentation

With the nightly version of `cargo doc` an additional index-page will be created.
```shell
$ RUSTDOCFLAGS="--enable-index-page -Zunstable-options" cargo +nightly doc --all --no-deps --document-private-items --all-features
```



#### 🔬 Testing

An continuous check on all files is done in the background and will trigger `cargo test` on each file change.

```shell
$ cargo watch --exec test
```

To run tests for all crates in this workspace, run:
```shell
$ cargo test --workspace --all-targets
```


### 📜 Roadmap (MvP)

 - Authenticate via openID
 - Create a new media item
 - Upload one or multiple photos
 - Download a list of owned media items
 - Download the original photo from a specific media item
 - *Metadata (size, resolution)
 - *EXIF (orientation, image taken timestamp, last modified, camera)
 - *RAW (image support for RAW-images)
 - *Resize (create low-resolutions / thumbnails)
 - *Deep learning (image recognition, Reinforcement learning)
 - *Plugin System (extract features into plugins)



### Visual Studio Code

The fastest start into development can be archived by using [Visual Studio Code](https://code.visualstudio.com/) and [Docker](https://www.docker.com/get-started).

1. Install [Docker](https://www.docker.com/get-started)
2. Install [Visual Studio Code](https://code.visualstudio.com/)
3. Install [Visual Studio Code Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
4. Clone and Open this repository in Visual Studio Code
5. Click the "Reopen in Container" Dialog
6. Launch **Photos.network** from the `RUN` window.

![VS Code with devcontainers](vscode.gif)



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
