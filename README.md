# numby

### dependencies

- [rust](https://www.rust-lang.org/)
- [podman](https://podman.io/) and [podman-compose](https://github.com/containers/podman-compose)
- [mise](https://mise.jdx.dev/) - `cargo install mise`

```bash
podman compose up -d

mise use
mise db-init

mise start
```
