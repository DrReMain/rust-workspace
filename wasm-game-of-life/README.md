```shell
# ./wasm-game-of-life
wasm-pack build

# ./wasm-game-of-life/pkg
yarn link

# ./wasm-game-of-life/www
yarn
yarn link wasm-game-of-life
yarn start
```