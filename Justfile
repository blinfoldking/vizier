dev:
  @cargo run -- run --config .vizier.toml

tui:
  @cargo run -- tui --base-url localhost:9999

run:
  @cargo run --release -- run --config .vizier.toml

docker:
  @docker-compose down && docker-compose up -d
