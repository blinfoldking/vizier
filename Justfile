run:
  @RUST_LOG=debug,serenity=off,tracing=off,hyper=off,rig=off,h2=off,rustls=off,reqwest=off,tungstenite=off cargo run -- run --config .vizier.toml


