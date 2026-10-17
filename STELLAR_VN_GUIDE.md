# Stellar VN Lab — Freighter + Soroban (Testnet)

A Vietnamese educational dashboard available at `/stellar-vn/index.html` when this Next.js repository is running. It is isolated in `public/stellar-vn` to avoid modifying upstream Stellar Laboratory code.

## Features
- Connect to Freighter and verify **Testnet** network passphrase.
- Read actual balance from Horizon Testnet.
- Request free *test* XLM using Friendbot.
- Build an ordinary native XLM payment; Freighter signs the XDR, then Horizon submits it.
- Open transactions in Stellar Expert's Testnet explorer.
- Compute SHA-256 on the client; separately deploy the Soroban anchor contract.

## Contract
`contracts/stellar-vn-anchor/src/lib.rs` defines `anchor(owner: Address, digest: BytesN<32>) -> bool` and `get(digest: BytesN<32>) -> Option<Anchor>`. It requires owner authorization and rejects overwriting an existing digest. This is a demonstrator, not an audited production financial contract.

Install Rust + Stellar CLI, then in `contracts/stellar-vn-anchor`:

```sh
cargo test
stellar contract build
stellar keys generate deployer --network testnet
stellar keys fund deployer --network testnet
stellar contract deploy --wasm target/wasm32v1-none/release/stellar_vn_anchor.wasm --source deployer --network testnet
```

For function invocation and wallet-auth flow, use https://lab.stellar.org/ with the deployed contract ID. The web dashboard currently **does not auto-submit Soroban invocations**; its SHA-256 panel calculates a digest locally for use in the Laboratory.

## Deploy
- Run existing application: `pnpm install && pnpm dev`
- Open `http://localhost:3000/stellar-vn/index.html`.
- Next.js requires a Node-capable hosting service. GitHub Pages cannot run a Next.js server application directly. This isolated `public/stellar-vn/index.html` can also be copied into a separate static GitHub Pages branch/folder, but the repo's GitHub Pages publication must be configured explicitly.
- Dashboard CDN dependencies need internet access and working CDN availability.

## Security and licensing
- Never collect wallet recovery phrases, passwords or secret keys.
- Only Testnet is enabled in the demo. Verify address, memo, amount, network and fees before approving in Freighter.
- Production should pin dependency versions, introduce CSP and comprehensive E2E tests, and obtain independent security review.
- Upstream Stellar Laboratory is Apache-2.0. Keep original attribution and license; this additive example is Apache-2.0 as well.
