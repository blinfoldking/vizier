install:
  @cargo install cargo-watch
  @echo -e "\\e[1;32minstalling core dependencies\\e[0m\n"
  cargo fetch
  @echo -e "\n\\e[1;32mDone\\e[0m"
  @echo -e "\n\\e[1;34minstalling webui dependencies\\e[0m\n"
  cd webui && npm i
  @echo -e "\n\\e[1;32mDone\\e[0m"

run:
  @cargo run -- run --config .vizier/config.yaml

run-no-python:
  @cargo run --no-default-features -- run --config .vizier/config.yaml

dev:
  cargo watch -s "just run"

dev-no-python:
  cargo watch -s "just run-no-python"

docker:
  @docker-compose down && docker-compose up -d

build:
  @cd webui && npm run build

release:
  @cargo build --release

release-no-python:
  @cargo build --release --no-default-features
