#!/bin/bash

# Start Ollama in the background
ollama serve &

# Wait for the Ollama server to be ready
until curl -s http://localhost:11434/api/tags > /dev/null; do
  sleep 2
done

# Pull the LLM Model model
ollama pull deepseek-r1:1.5b

# Pull embedding model
ollama pull nomic-embed-text

# Keep the container running
wait
