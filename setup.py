"""Photos.network setup script"""
from setuptools import setup

from core import const

import sys

if sys.version_info < (3, 0):
    print("{PROJECT_NAME} requires python version >= 3.0")
    sys.exit(1)

setup(
    name="core",
    version=const.CORE_VERSION,
    description="The core system for photos.network",
    long_description="The core system for photos.network to manage components.",
    author="The Photos Network Authors",
    author_email="devs@photos.network",
    url="https://dev.photos.network/core",
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
        "core": ["addons/**/*.py","addons/**/model/**"],
    }
)
