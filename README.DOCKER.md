This repository includes a production-ready Dockerfile and a simple `docker-compose.yml` service to run the Vector Search API (Rust + SPFresh).

Quick notes and recommendations

1) SPFresh native libraries

- The image build tries to copy `SPFresh/SPFresh/Release` from the repo into `/usr/local/lib/spfresh-release` inside the image. If you don't have that folder committed to the repository (we previously used it locally), you should mount it at runtime via the volume in `docker-compose.yml`:

  volumes:
    - ./SPFresh/SPFresh/Release:/usr/local/lib/spfresh-release:ro

- Alternatively, you can copy the Release folder into the repo before building the image so the native libs are baked into the image.

2) Model downloads

- fastembed (the embedding runtime) downloads the model files on first run. For production it's recommended to pre-download the model and either:
  - Bake it into the image (copy model files into the image during build), or
  - Provide a startup step that downloads the model into a shared volume before starting the server.

3) Build & run

- Build the image locally:

```bash
# Build with Docker
docker build -t vector-search-api:latest .

# Or use docker-compose to build and run
docker compose up --build -d
```

- If you prefer to mount SPFresh Release at runtime (instead of baking): edit `docker-compose.yml` and uncomment the volume line for `SPFresh/SPFresh/Release`.

4) Environment & configuration

- The server reads `VECTOR_CONFIG_PATH` env var if set; otherwise it will try `./config.toml` then `./config.json` and fall back to defaults. The Dockerfile copies `config.json` from the repo root into the image as `/app/config.json`. Override with an env var if you want a different path.

5) Data persistence

- `docker-compose.yml` mounts `./data` to `/app/data` so your append-only JSONL and index files persist across container restarts.

6) Runtime library path

- The image sets `LD_LIBRARY_PATH=/usr/local/lib/spfresh-release` so the SPFresh shared libs can be resolved at runtime. If you mount the folder elsewhere, set `LD_LIBRARY_PATH` accordingly.

7) Healthcheck

- The image exposes a Docker HEALTHCHECK that hits `/health` on port 3000. Adjust if you change the server binding.

If you want, I can:
- Add pre-download of the embedding model into the image (requires adding model files or download step to the build stage).
- Bake SPFresh Release into the image (if you confirm the folder is present and allowed to be included).
- Create a small systemd / Kubernetes deployment manifest for production.
