FROM ghcr.io/dlrobson/rust-ci:1.93.1-0

USER root

# Install Node.js and npm; also install pnpm (required by crux_core TypeScript codegen)
RUN curl -fsSL https://deb.nodesource.com/setup_24.x | bash - && \
    apt-get install -y nodejs && \
    npm install -g npm@latest pnpm@latest && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

USER ${USER}

COPY --chown=${USER}:${USER} binstall-versions.json /tmp/binstall-versions.json
RUN cargo binstall --no-confirm $(jq -r '.[] | "\(.crate)@\(.version)"' /tmp/binstall-versions.json) && \
    rm /tmp/binstall-versions.json
