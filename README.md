# Photos.network

[![License](https://img.shields.io/github/license/photos.network/core)](./LICENSE.md)
[![GitHub contributors](https://img.shields.io/github/contributors/photos.network/core)](https://github.com/photos.network/core/graphs/contributors)

[Photos.network](https://photos.network) is an open source project for self hosted photo management.
Its core features are:
 - Share photos with friends, family or public
 - Filter / Search photos by attributes like location or date
 - Group photos by objects like people of objects

## Core
This repository contains the core system of the project.
It is responsible for main tasks e.g.:
 - **Authentication**  (validate the identity of users)
 - **Authorization**  (handle access privileges of resources like photos or albums)
 - **Add-on Handling**  (managing add-ons)
 - **Persistency**  (read / write data)
 - **Task Processing**  (keep track of running tasks)

### Add-ons
The core system can be extended by add-ons to attune user needs.

Each addon is encapsulated in a separate directory wich have to contain at least:
- `addon.json` with detailed informations and requirements
- `__init__.py` with an async setup function

```json
{
  "domain": "api",
  "name": "REST api",
  "requirements": [],
  "core": "0.1.0"
}
```

```python
async def async_setup(core: ApplicationCore, config: dict) -> bool:
    return True
```


 - Sync photos (reading & writing)
   - server <=> server
     - nextcloud, gdrive, icloud
   - client <=> server
     - browser, native app
 - File processing
   - Metadata parsing
 - Image processing
   - Object detection
   - Face detection



## Development
Always use [PEP 484: Type Hints](https://www.python.org/dev/peps/pep-0484/) in your syntax.

#### Prepare setup
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

#### Run
```shell
python3 ./venv/bin/core
```
