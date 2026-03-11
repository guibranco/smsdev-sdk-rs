# API Client boilerplate template

A boilerplate API client template for those who want to build Rust API client wrappers

<!-- Remove the following lines until the following comment section -->

## Instructions on how to use this template

> **Note**
> - Run the `initial-setup.bat` and follow the prompts
>
> OR
>
> -  Change the *appveyor.yml* with your **tokens** (use secure tokens) or use your preferred CI tool (Circle CI, GitHub Actions, Jenkins, Team City, Azure DevOps)
> -  Fix in the *readme* the path of the badges and remove this section. (change all *{username}/{repo}* tokens with your own data).
> -  Add to *readme* usage instructions of your client/SDK.
> -  Update _config.yml with project details (github.io docs website)

<!-- remove lines until this line -->

# {Project Name} SDK Rust

[![wakatime](https://wakatime.com/badge/github/{username}/{repo}.svg)](https://wakatime.com/badge/github/{username}/{repo})
[![Maintainability](https://api.codeclimate.com/v1/badges/{codeClimateId}/maintainability)](https://codeclimate.com/github/{username}/{repo}/maintainability)
[![Test Coverage](https://api.codeclimate.com/v1/badges/{codeClimateId}/test_coverage)](https://codeclimate.com/github/{username}/{repo}/test_coverage)
[![CodeFactor](https://www.codefactor.io/repository/github/{username}/{repo}/badge)](https://www.codefactor.io/repository/github/{username}/{repo})

| Service      | Status |
| -------      | :----: |
| AppVeyor CI  | [![Build status](https://ci.appveyor.com/api/projects/status/{appVeyorId}?svg=true)](https://ci.appveyor.com/project/{username}/{repo}/branch/main) |
| crates.io    | [![crates.io](https://img.shields.io/crates/v/package-name-rs.svg)](https://crates.io/crates/package-name-rs) |

{{Project Description}}

---

## Dependencies and support

**package-name-rs** is intended to work on all tier 1 supported Rust systems:

- MacOSX
- Linux
- Windows

---

## Minimum Compiler Version

Due to the use of certain features, package-name-rs requires `rustc` version 1.18 or
higher.

---

## Getting Started

Add the following to your `Cargo.toml`

```toml
[dependencies]
package_name_rs = "0.0.1"
serde_json = "1.0"
```

Then in your `lib.rs` or `main.rs` file add:
```rust
extern crate package_name;

let client = PackageNameAPIClient::new();

```

---

## License

Licensed under

MIT license ([LICENSE](https://github.com/{{username}}/{{repository}}/blob/main/LICENSE) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
