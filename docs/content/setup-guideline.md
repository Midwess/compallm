---
title: Setup
order: 1
available: true
---

# Setup

## Status

This deployment guide is a draft. Authentication and initial-organization environment variables are planned and are not consumed by the application yet.

## Requirements

- Docker Engine with Docker Compose v2
- A host name or IP address that users can reach
- An SMTP account, GitHub OAuth application, or GitLab OAuth application when the corresponding login method is implemented and enabled

## Create the deployment files

Create an empty directory for the deployment and add the following `compose.yaml`:

```yaml
services:
  compallm:
    image: docker.io/library/midwesshub/compallm
    restart: unless-stopped
    ports:
      - "3000:3000"
    environment:
      PORT: 3000
      LOG_LEVEL: info
      DATABASE_URL: postgres://compallm:replace-with-a-random-password@postgres:5432/compallm
    depends_on:
      postgres:
        condition: service_healthy

  postgres:
    image: docker.io/library/postgres:18-alpine
    restart: unless-stopped
    environment:
      POSTGRES_USER: compallm
      POSTGRES_PASSWORD: replace-with-a-random-password
      POSTGRES_DB: compallm
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U compallm -d compallm"]
      interval: 5s
      timeout: 5s
      retries: 10
    volumes:
      - postgres-data:/var/lib/postgresql

volumes:
  postgres-data:
```

Replace both occurrences of `replace-with-a-random-password` with the same long, randomly generated value containing only letters and numbers. Restrict access to `compose.yaml` because it contains the database password, and do not commit it to version control.

## Compallm environment variables

### Runtime

These variables are supported by the current production configuration:

| Variable | Required | Default | Purpose |
| --- | --- | --- | --- |
| `DATABASE_URL` | Yes | None | PostgreSQL connection URL |
| `PORT` | No | `3000` | Preferred HTTP starting port; Hero selects the first available port at or above it, up to `50100` |
| `LOG_LEVEL` | No | `info` | Application log level |

The password in `DATABASE_URL` must match `POSTGRES_PASSWORD`. Use only letters and numbers so it is safe in the connection URL without encoding.

### Planned authentication

The following names document planned configuration and are not implemented yet. Add only the variables required by the login methods you enable after authentication support is released.

| Variable | Required for | Purpose |
| --- | --- | --- |
| `SMTP_HOST` | Email code | SMTP server hostname |
| `SMTP_PORT` | Email code | SMTP server port, commonly `587` for message submission |
| `SMTP_USERNAME` | Email code | SMTP authentication username |
| `SMTP_PASSWORD` | Email code | SMTP authentication password |
| `SMTP_FROM_ADDRESS` | Email code | Sender address for login-code messages |
| `SMTP_TLS` | Email code | Whether the configured SMTP connection requires TLS |

The initial organization will also be provisioned from environment variables on the first deployment. The variable names and required fields remain to be defined.

## Start Compallm

Pull the configured images:

```bash
docker compose pull
```

The production configuration does not automatically apply database migrations. The release image must document its migration command before this guide can provide the exact one-off command.

After applying migrations, start the services:

```bash
docker compose up -d
```

Check container state and follow application logs:

```bash
docker compose ps
docker compose logs -f compallm
```

The API logs its selected listening port and listens on all IPv4/IPv6 interfaces. With the Compose mapping above, keep container port `3000` available and open `http://server.example.com:3000`. If the server selects another port, update the container mapping or reverse-proxy target to match it.

## Update

Pull the latest configured image and recreate the containers:

```bash
docker compose pull
docker compose up -d
```

Read the release notes and apply required database migrations before starting a newer application version.

## Stop or remove the deployment

Stop the containers while preserving PostgreSQL data:

```bash
docker compose down
```

The `postgres-data` volume contains the database. Deleting that volume permanently deletes the deployment data.

## Related pages

- [Login](login/adr.md)
- [Email login](login/email.md)
- [GitHub login](login/github.md)
- [GitLab login](login/gitlab.md)
- [Development](development/adr.md)
