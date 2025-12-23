FROM alpine:latest

# Install dependencies
RUN apk add --no-cache \
    git \
    curl \
    wget \
    build-base \
    nodejs \
    npm \
    python3 \
    py3-pip \
    ripgrep \
    neovim \
    lsof

# Install opencode
RUN curl -fsSL https://opencode.ai/install | bash

# Set up user
RUN adduser -D neovim
USER neovim
WORKDIR /home/neovim

# Copy Neovim config
COPY --chown=neovim:neovim . /home/neovim/.config/nvim

# Install plugins (lazy.nvim will handle this)
RUN nvim --headless -c 'Lazy sync' -c 'qa'

# Set entrypoint
ENTRYPOINT ["nvim"]