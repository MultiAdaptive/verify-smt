# Build and test for program compiled natively

### Environment Setup

* Install Rust from https://rustup.rs/

```
$ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10) 
$ cargo --version
cargo 1.79.0 (ffa9cf99a 2024-06-03)  
```

```
$ cargo build
$ cargo test
```

# Build and test for program compiled onchain

### Environment Setup

* Install Solana 1.18.12 from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

```
$ solana --version
solana-cli 1.18.12 (src:b9c13825; feat:4215500110, client:SolanaLabs)
```

```
$ cargo build-sbf
$ cargo test-sbf
```
