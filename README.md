## Como buildar
Tenha o trunk instalado
```bash
cargo install trunk
```
Eu tentei fazer com que o repo baixasse a nightly do rust automaticamente, caso não tenha funcionado, o comando a seguir faz isso:
```bash
rustup toolchain install nightly
rustup override set nightly # Dentro do diretório do projeto
rustup target add wasm32-unknown-unknown
```
Agora só rodar:
```bash
trunk serve --open
```
