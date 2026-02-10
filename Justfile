run:
  @RUST_LOG=debug,serenity=off,tracing=off,hyper=off,h2=off,rustls=off,reqwest=off,tungstenite=off,rig=off cargo run -- run --config .vizier.toml

docker:
  @docker-compose down && docker-compose up -d
