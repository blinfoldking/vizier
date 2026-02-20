dev:
  @cargo run -- run --config .vizier.toml

run:
  @cargo run --release -- run --config .vizier.toml

docker:
  @docker-compose down && docker-compose up -d
