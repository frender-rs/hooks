# Hooks

Compile-time, async hooks in safe Rust.

_Working in progress._

## Development

This project uses [Nx](https://nx.dev)
with [`@nxrs/cargo`](https://github.com/nxrs/cargo)
and [@theunderscorer/nx-semantic-release](https://github.com/TheUnderScorer/nx-semantic-release) for versioning and automatic [semantic releasing](https://github.com/semantic-release/semantic-release).

To generate a new rust lib, run:

```sh
yarn nx g @nxrs/cargo:lib $PROJECT_NAME
yarn nx g @theunderscorer/nx-semantic-release:setup-project $PROJECT_NAME
```
