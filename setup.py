"""Photos.network setup script"""
import sys

from setuptools import setup

from core import const

if sys.version_info < (3, 0):
    print("{PROJECT_NAME} requires python version >= 3.0")
    sys.exit(1)

setup(
    name="core",
    version=const.CORE_VERSION,
    description="The core system for photos.network",
    long_description="The core system for photos.network to manage components.",
    author="The Photos.network Authors",
    author_email="devs@photos.network",
    url="https://developers.photos.network/core/",
    license="Apache License 2.0",
    classifiers=[
        "Intended Audience :: End Users/Desktop",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Topic :: Software Development :: Libraries :: Python Modules",
        "Topic :: Scientific/Engineering :: Atmospheric Science",
        "Development Status :: 5 - Production/Stable",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3.8",
    ],
    keywords=["docker", "photos-network", "api"],
    zip_safe=False,
    platforms="any",
    packages=[
        "core",
        "core.addons",
        "core.authentication",
        "core.authorization",
        "core.persistency",
        "core.utils",
        "core.webserver",
    ],
    entry_points={"console_scripts": ["core = core.__main__:main"]},
    include_package_data=True,
    package_data={
        "core": ["addons/**/*.py", "addons/**/model/**", "addons/**/dto/**"],
    },
    install_requires=[
        "async_timeout>=3.0.1,<4.0",
        "aiohttp>=3.7.0,<4.0",
        "aiohttp_cors>=0.7.0",
        "aiohttp-jinja2>=1.4.2",
        "oauth2-stateless>=1.1.0",
        "colorlog>=4.0.0",
        "jinja2>=2.11.2",
        "pip>=8.0.3",
        "pytz>=2020.1",
        "pyyaml>=5.3.1",
        "sqlalchemy>=1.3.20,<1.4",
        "voluptuous>=0.12.0",
        "voluptuous-serialize>=2.4.0",
        "wheel>=0.36.2",
    ],
)
