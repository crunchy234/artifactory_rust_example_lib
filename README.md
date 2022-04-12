# Artifactory Rust Example Lib
First set `~/.cargo/config.toml` to point to your Artifactory server.

```toml
[registries.artifactory]
index = "https://placeholder.jfrog.io/artifactory/git/repository-name.git"

# For sending credentials in git requests.
# Not required if anonymous access is enabled
[net]
git-fetch-with-cli = true
```

Also set `~/.cargo/credentials.toml` as per the artifactory "Set Me Up" instructions.

## Deploy the library

```bash
cargo publish
```