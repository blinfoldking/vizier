# Use the official Ollama base image
FROM ollama/ollama:latest

RUN apt-get update

RUN apt-get install -y curl

# Set the working directory
WORKDIR /app

# Copy the entrypoint script
COPY entrypoint.sh entrypoint.sh

# Give execution rights to the script
RUN chmod +x entrypoint.sh

# Expose the default Ollama port
EXPOSE 11434

# Use the script to start the server and pull the model
ENTRYPOINT ["./entrypoint.sh"]
