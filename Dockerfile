FROM docker.nix-community.org/nixpkgs/nix-flakes:latest as builder
WORKDIR /src
COPY . .

RUN nix develop .#ci --option filter-syscalls false -c true
RUN nix build .#default.cargoDeps --no-link --option filter-syscalls false
RUN nix build --option filter-syscalls false

FROM scratch
WORKDIR /app
COPY --from=builder /src/result/bin/resty-kv /bin/resty-kv
ENTRYPOINT ["/bin/resty-kv"]
