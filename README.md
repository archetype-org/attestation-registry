# Attestation Registry for BOS Components

This smart contract tracks IPFS hashes that can contain metadata pertaining to BOS components that authors can update or create seperate releases. It utilizes the lazy-loaded storage 
trie similar to NEAR SocialDB. In addition to this, arbitrary users can attest to data that map via the package name and the account ID of the author. To save on gas, attestors who
attempt to make claims against packages *not* already in the registry will revert. Furthermore `content_type` can be specified for releases to allow consumers different methods of resolution once the content is retrieved.

# Quickstart

## Build, Test and Deploy
To build the contract you can execute the `./build.sh` script, which will in turn run:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Then, run the `./deploy.sh` script, which will in turn run:

```bash
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/attestation-registry.wasm
```

To run unit tests seperately, execute `./test.sh`

Latest version has been deployed via testnet [here](https://explorer.testnet.near.org/accounts/dev-1706709131163-75127504488588).
