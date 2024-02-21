# Attestation Registry for BOS Components

This smart contract tracks IPFS hashes that can contain metadata pertaining to BOS components that authors can update or create seperate releases. It utilizes the lazy-loaded storage 
trie similar to NEAR SocialDB. In addition to this, arbitrary users can attest to data that map via the package name and the account ID of the author. To save on gas, attestors who
attempt to make claims against packages *not* already in the registry will revert. Furthermore `content_type` can be specified for releases to allow consumers different methods of resolution once the content is retrieved.

## Usage

:information_source: A JS library will be provided to interact with the contract, but for now, here is an example NEAR CLI command.

To publish a package manifest to the registry, use the following command with `near-cli-rs`, replacing with your own values:

```bash
near contract \
    call-function \
    as-transaction {{reg.archetype-test.testnet}} create_manifest \
    json-args '{
      "package_name": "test-package",
      "version": "0.0.1",
      "content_type": "ipfs",
      "cid": "bafybeicn7i3soqdgr7dwnrwytgq4zxy7a5jpkizrvhm5mv6bgjd32wm3q4"
      }' \
    prepaid-gas '3 Tgas' \
    attached-deposit '0 NEAR' \
    sign-as {{mrtesterman.testnet}} \
    network-config testnet \
    sign-with-keychain \
    send
```

## Development

### Prerequisites

- [rustup](https://rustup.rs/)
- [cargo-near](https://github.com/near/cargo-near)
- [near-cli-rs](https://github.com/near/near-cli-rs) OR [near-cli](https://github.com/near/near-cli)

### Build, Test and Deploy
To build the contract you can execute the `./build.sh` script, which will in turn run:

```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Then, run the `./deploy.sh` script, which will in turn run:

```bash
near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/attestation_registry.wasm
```

To run unit tests seperately, execute `./test.sh`

Latest version has been deployed via testnet [here](https://explorer.testnet.near.org/accounts/dev-1706709131163-75127504488588).

If you're using `near-cli-rs`, you can deploy with this command:

```bash
near contract deploy {{reg.archetype-test.testnet}} use-file ./target/wasm32-unknown-unknown/release/attestation_registry.wasm without-init-call network-config {{testnet}}
```

