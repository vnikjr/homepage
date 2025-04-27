FROM nixos/nix

# Enable flakes and experimental features
ENV NIX_CONFIG="experimental-features = nix-command flakes"

# Create a working directory
WORKDIR /workspace

# Copy your flake into the container
COPY . /workspace

# Optionally: prefetch and build the flake to cache dependencies
RUN nix develop --command cargo build -r


# Default to starting an interactive shell in the flake's dev shell
CMD [ "nix", "develop", "--command", "cargo", "run", "-r" ]
