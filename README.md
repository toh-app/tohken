# Toh! Redeem

This is the repository containing the smart contract code for the Toh! app.

It currently supports wallet connection and tohken redemption.

## Getting started

- Install the toolchain, for quick guide check out [the Anchor docs](https://www.anchor-lang.com/docs/installation).
  - Rust 1.60.x
  - Anchor 0.27.0
  - NodeJS 16.x
  - Yarn 1.22.x
- Recomended if you use vscode: [Rust official vscode extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
- Install node_modules with `yarn`
- Update anchor provider.wallet on `Anchor.toml` file with the path to your [keypair](https://docs.solana.com/wallet-guide/file-system-wallet#generate-a-file-system-wallet-keypair). You can check your keypair path with running `yarn keypair`

## Commands

- Build the program with `anchor build`
- Deploy the program with `anchor deploy`

## Getting Program ID

```bash
anchor build
anchor keys list
```