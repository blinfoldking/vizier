install:
  @cargo install cargo-watch
  @echo -e "\\e[1;32minstalling core dependencies\\e[0m\n"
  cargo fetch
  @echo -e "\n\\e[1;32mDone\\e[0m"
  @echo -e "\n\\e[1;34minstalling webui dependencies\\e[0m\n"
  cd webui && npm i
  @echo -e "\n\\e[1;32mDone\\e[0m"

run:
  @cargo run -- run --config dev.vizier.yaml

run-python:
  @cargo run --features python -- run --config dev.vizier.yaml

dev:
  cargo watch -s "just run"

dev-python:
  cargo watch -s "just run-python"

docker:
  @docker-compose down && docker-compose up -d

build:
  @cd webui && npm run build

release:
  @cargo build --release

release-python:
  @cargo build --release --features python
