# database

This crate provides an database abstraction used within [Photos.network](https://photos.network).

## Framework choice

The decision for [sea-orm](https://www.sea-ql.org/SeaORM/) has 3 main reasons compared to [Diesel](https://diesel.rs/)

- supports async
- testable
- written in rust

A big downsite of sea-orm:

- performance
