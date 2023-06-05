# Usage

Let enums derive `prost_enum::Serialize_enum` and/or `prost_enum::Deserialize_enum` instead of default ones of serde.

```rust
// In build.rs
let mut config = prost_build::Config::new();
// ...
config.enum_attribute(
    ".",
    "#[derive(prost_enum::Serialize_enum, prost_enum::Deserialize_enum)]",
);
// ...
```
