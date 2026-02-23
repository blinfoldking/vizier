install:
  @echo -e "\\e[1;32minstalling core dependencies\\e[0m\n"
  cargo fetch
  @echo -e "\n\\e[1;32mDone\\e[0m"
  @echo -e "\n\\e[1;34minstalling webui dependencies\\e[0m\n"
  npm i -g concurrently
  cd webui && npm i
  @echo -e "\n\\e[1;32mDone\\e[0m"

dev:
  @concurrently 'just serve' 'just webui'

webui:
  @cd webui && npm run dev

serve:
  @cargo run -- run --config .vizier.toml

tui:
  @cargo run -- tui --base-url localhost:9999

run:
  @cargo run --release -- run --config .vizier.toml

docker:
  @docker-compose down && docker-compose up -d

build:
  @cd webui && npm run build
