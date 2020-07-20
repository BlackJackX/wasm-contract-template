## Description
一个wasm合约模板，目前由于调用了serde模块导致无法部署，如果不导入serde模块，需要将pwasm-std中panic模块解除注释，或者从crates.io导入pwasm相关包

## Build
Make sure you've installed [required tools](https://github.com/paritytech/pwasm-tutorial/blob/master/README.md#tutorial-prerequisites)
```
./build.sh
```
As a result the `pwasm_tutorial_contract.wasm` should be created under the `step-5/target/wasm32-unknown-unknown/release/` directory.

## Deploy
See https://github.com/fckt/pwasm-tutorial#deploy

## Test
```
cargo test --features std
```
