# Usage

Use `prost_enum::enhance` to make protobuf enums support better serialize / deserialize.
(Optional) Make enums to be used in Sea-ORM, which will be treated as `i32` / `Integer`.

```rust
// In build.rs
let mut config = prost_build::Config::new();
// ...
config.enum_attribute(
    ".",
    "#[prost_enum::enhance]",
);
// ...
```
