# soroban contract install \
#   --network testnet \
#   --source alice \
#   --wasm target/wasm32-unknown-unknown/release/hello_world.wasm
soroban contract build
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello_world.wasm \
  --source alice \
  --network testnet

