# reliquary

a library to parse network packets from a certain turn based anime game!

| module     | purpose                                      |
|------------|----------------------------------------------|
| `network`  | parse bytes into network packets             |
| `resource` | look up network packet resource ids from SRD |

> [!WARNING]
> Many field names are obfuscated due to lacking a good name translation source for v2.6+

## use in your projects

not published to crates.io out of caution. add following to your Cargo.toml to use

```toml
[dependencies]
reliquary = { git = "https://github.com/IceDynamix/reliquary" } # optionally add revision
```

for documentation, use `cargo doc`

## codegen

types are outdated? check out [reliquary-codegen](https://github.com/IceDynamix/reliquary-codegen)

## versioning

different package versions were made for different game versions. all game version updates will warrant breaking changes in the generated protobuf types, hence the major version bumps.

| package version | game version |
|-----------------|--------------|
| `2.0.0`         | `2.3`        |
| `3.1.0`         | `2.4`        |
| `4.0.0`         | `2.5`        |
| `5.1.0`         | `2.6`        |
| `6.2.1`         | `2.7`        |
| `7.0.0`         | `3.0`        |

## related

- [reliquary-archiver](https://github.com/IceDynamix/reliquary-archiver)
