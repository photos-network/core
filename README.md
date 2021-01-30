# Photos.network

[![License](https://img.shields.io/github/license/photos-network/core)](./LICENSE.md)
[![GitHub contributors](https://img.shields.io/github/contributors/photos-network/core?color=success)](https://github.com/photos-network/core/graphs/contributors)
[![Discord](https://img.shields.io/discord/793235453871390720)](https://discord.gg/dGFDpmWp46)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/photos-network/core/check%20code%20quality)


[Photos.network](https://photos.network) is an open source project for self hosted photo management.
Its core features are:

- Share photos with friends, family or public
- Filter / Search photos by attributes like location or date
- Group photos by objects like people of objects

## Core

This repository contains the core system of the project.
It is responsible for main tasks e.g.:

- **Authentication** (validate the identity of users)
- **Authorization** (handle access privileges of resources like photos or albums)
- **Add-on Handling** (managing add-ons)
- **Persistency** (read / write data)
- **Task Processing** (keep track of running tasks)

## Installation

Install core dependencies:

```shell
$ pip3 install -r requirements.txt
```

Install the core system from the wheel

```shell
$ pip3 install --upgrade --force-reinstall core-0.1.0-py3-none-any.whl
```

## Development

Always use [PEP 484: Type Hints](https://www.python.org/dev/peps/pep-0484/) in your syntax.

### Visual Studio Code

The fastest start into development can be archived by using [Visual Studio Code](https://code.visualstudio.com/) and [Docker](https://www.docker.com/get-started).

1. Install [Docker](https://www.docker.com/get-started)
2. Install [Visual Studio Code](https://code.visualstudio.com/)
3. Install [Visual Studio Code Remote - Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)
4. Clone and Open this repository in Visual Studio Code
5. Click the "Reopen in Container" Dialog
6. Launch **Photos.network** from the `RUN` window.

### Manual Environment

Prepare an environment by running:

```shell
python3 -m venv venv
source ./venv/bin/activate
pip3 install -r requirements_dev.txt
```

After the environment is build, install the core:

```shell
python3 setup.py install
```

## Run

```shell
python3 ./venv/bin/core
```
