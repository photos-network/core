# database

This crate provides an database abstraction used within [Photos.network](https://photos.network).


We're using polymorphism via trait objects so users can choose between differen database implementations like `PostgreSQL`, `MySQL` or `SQLite`.
The [trait object](lib.rs) defines shared behaviour and is implemented multiple times for each database type.

Another solution would be to use a generic type, since we only need a single instance for now, it would be totally sufficient.
It might be possible that a user wants to migrate from a `SQLite` to a `PostgreSQL` in the future, than it would be a hard limitation to use only
a single generic database.
