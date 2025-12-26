# anyform Phase 4.1: Distribution

> **Status**: In Progress (release checklist pending)
> **PRD Reference**: [axum-sea-forms-prd-0.4.0.md](/Users/rick/p/wordpuppi/docs/prd/libs/asf/axum-sea-forms-prd-0.4.0.md) Section 4
> **Depends On**: Phase 4.0 (complete), Phase 4.3 WASM (complete)

---

## Overview

Phase 4.1 delivers anyform to users via multiple distribution channels:
- **GitHub Releases** - Cross-platform binaries with checksums
- **Docker** - Container images on Docker Hub and GHCR
- **Homebrew** - macOS package manager
- **Scoop** - Windows package manager
- **Install Script** - Curl-based installer for Linux/macOS

---

## 1. GitHub Actions Release Workflow

**File:** `.github/workflows/release.yml`

### 1.1 Update Build Matrix
- [x] Change package from `axum-sea-forms-cli` to `anyform`
- [x] Rename artifact names from `asf-*` to `anyform-*`:
  - `asf-linux-x86_64` → `anyform-linux-amd64`
  - `asf-macos-x86_64` → `anyform-darwin-amd64`
  - `asf-macos-aarch64` → `anyform-darwin-arm64`
  - `asf-windows-x86_64` → `anyform-windows-amd64.exe`
- [x] Add Linux ARM64 target:
  ```yaml
  - target: aarch64-unknown-linux-gnu
    os: ubuntu-latest
    name: anyform-linux-arm64
    cross: true
  ```
- [x] Install `cross` for Linux ARM64 cross-compilation

### 1.2 Add Checksum Generation
- [x] Add step to generate SHA256 checksums:
  ```yaml
  - name: Generate checksums
    run: |
      cd artifacts
      sha256sum */anyform-* > checksums.txt
  ```
- [x] Include `checksums.txt` in release assets

### 1.3 Update Release Job
- [x] Update artifact file paths to use new names
- [x] Add checksums.txt to release files

### 1.4 Update Docker Job
- [x] Change image tags from `wordpuppi/axum-sea-forms` to `epenabella/anyform`
- [x] Add GitHub Container Registry (ghcr.io):
  ```yaml
  tags: |
    epenabella/anyform:${{ steps.version.outputs.VERSION }}
    epenabella/anyform:latest
    ghcr.io/${{ github.repository }}:${{ steps.version.outputs.VERSION }}
    ghcr.io/${{ github.repository }}:latest
  ```
- [x] Add GHCR login step

### 1.5 Add WASM Build Job (Optional for 4.1)
- [x] Add wasm-pack build job
- [x] Publish to npm (requires npm token secret)

---

## 2. Docker

### 2.1 Update Dockerfile
**File:** `Dockerfile`

- [x] Update build command:
  ```dockerfile
  # Change from:
  RUN cargo build --release --package axum-sea-forms-cli
  # To:
  RUN cargo build --release --package anyform --features cli
  ```
- [x] Update binary copy:
  ```dockerfile
  # Change from:
  COPY --from=builder /app/target/release/asf /usr/local/bin/asf
  # To:
  COPY --from=builder /app/target/release/anyform /usr/local/bin/anyform
  ```
- [x] Update CMD:
  ```dockerfile
  CMD ["anyform", "serve", "--host", "0.0.0.0"]
  ```
- [x] Keep `debian:bookworm-slim` as runtime base (glibc compatible)

### 2.2 Create docker-compose.yml
**File:** `docker-compose.yml`

- [x] Create example with embedded SQLite:
  ```yaml
  services:
    anyform:
      image: epenabella/anyform:latest
      ports:
        - "3000:3000"
      volumes:
        - anyform-data:/data
  volumes:
    anyform-data:
  ```

### 2.3 Create docker-compose.postgres.yml
**File:** `docker-compose.postgres.yml`

- [x] Create example with PostgreSQL:
  ```yaml
  services:
    anyform:
      image: epenabella/anyform:latest
      ports:
        - "3000:3000"
      environment:
        DATABASE_URL: postgres://anyform:anyform@db:5432/anyform
      depends_on:
        - db
    db:
      image: postgres:16-alpine
      environment:
        POSTGRES_USER: anyform
        POSTGRES_PASSWORD: anyform
        POSTGRES_DB: anyform
      volumes:
        - postgres-data:/var/lib/postgresql/data
  volumes:
    postgres-data:
  ```

### 2.4 Test Docker Images
- [x] Test `docker build .` locally
- [x] Test `docker run` with embedded SQLite
- [ ] Test docker-compose with PostgreSQL (deferred)

---

## 3. Install Script

### 3.1 Create install.sh
**File:** `install.sh`

- [x] Create shell script with:
  ```bash
  #!/bin/sh
  set -e

  # Detect OS
  OS=$(uname -s | tr '[:upper:]' '[:lower:]')
  ARCH=$(uname -m)

  # Map architecture
  case "$ARCH" in
    x86_64) ARCH="amd64" ;;
    aarch64|arm64) ARCH="arm64" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
  esac

  # Map OS
  case "$OS" in
    linux) OS="linux" ;;
    darwin) OS="darwin" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
  esac

  # Get latest version
  VERSION=$(curl -s https://api.github.com/repos/epenabella/anyform/releases/latest | grep tag_name | cut -d '"' -f 4)

  # Download
  BINARY="anyform-${OS}-${ARCH}"
  URL="https://github.com/epenabella/anyform/releases/download/${VERSION}/${BINARY}"

  echo "Downloading anyform ${VERSION} for ${OS}/${ARCH}..."
  curl -sL "$URL" -o /tmp/anyform
  chmod +x /tmp/anyform

  # Install
  sudo mv /tmp/anyform /usr/local/bin/anyform
  echo "anyform installed to /usr/local/bin/anyform"
  anyform --version
  ```

### 3.2 Add Checksum Verification
- [x] Download and verify checksums:
  ```bash
  curl -sL "${URL}.sha256" -o /tmp/anyform.sha256
  cd /tmp && sha256sum -c anyform.sha256
  ```

### 3.3 Host Install Script
- [ ] Option A: GitHub Pages at anyform.dev
- [x] Option B: Raw GitHub URL from main branch

---

## 4. Package Managers

### 4.1 Homebrew Formula
**File:** `homebrew/anyform.rb`

- [x] Create formula:
  ```ruby
  class Anyform < Formula
    desc "Any database. Any form. Zero hassle."
    homepage "https://github.com/epenabella/anyform"
    version "0.4.0"
    license "MIT"

    on_macos do
      on_intel do
        url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-darwin-amd64"
        sha256 "CHECKSUM_HERE"
      end
      on_arm do
        url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-darwin-arm64"
        sha256 "CHECKSUM_HERE"
      end
    end

    on_linux do
      on_intel do
        url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-linux-amd64"
        sha256 "CHECKSUM_HERE"
      end
      on_arm do
        url "https://github.com/epenabella/anyform/releases/download/v#{version}/anyform-linux-arm64"
        sha256 "CHECKSUM_HERE"
      end
    end

    def install
      bin.install "anyform-*" => "anyform"
    end

    test do
      system "#{bin}/anyform", "--version"
    end
  end
  ```

- [ ] Create Homebrew tap repo: `epenabella/homebrew-tap` (deferred)
- [ ] Or submit to homebrew-core (after project matures)

### 4.2 Scoop Manifest
**File:** `scoop/anyform.json`

- [x] Create manifest:
  ```json
  {
    "version": "0.4.0",
    "description": "Any database. Any form. Zero hassle.",
    "homepage": "https://github.com/epenabella/anyform",
    "license": "MIT",
    "architecture": {
      "64bit": {
        "url": "https://github.com/epenabella/anyform/releases/download/v0.4.0/anyform-windows-amd64.exe",
        "hash": "CHECKSUM_HERE"
      }
    },
    "bin": "anyform-windows-amd64.exe",
    "checkver": "github",
    "autoupdate": {
      "architecture": {
        "64bit": {
          "url": "https://github.com/epenabella/anyform/releases/download/v$version/anyform-windows-amd64.exe"
        }
      }
    }
  }
  ```

- [ ] Create Scoop bucket repo: `epenabella/scoop-anyform` (deferred)
- [ ] Or submit to scoop-extras bucket

---

## 5. Documentation

### 5.1 Update README.md
- [x] Add Installation section:
  ```markdown
  ## Installation

  ### macOS (Homebrew)
  ```bash
  brew install epenabella/tap/anyform
  ```

  ### Linux (curl)
  ```bash
  curl -fsSL https://raw.githubusercontent.com/epenabella/anyform/main/install.sh | sh
  ```

  ### Windows (Scoop)
  ```powershell
  scoop bucket add anyform https://github.com/epenabella/scoop-anyform
  scoop install anyform
  ```

  ### Docker
  ```bash
  docker run -p 3000:3000 epenabella/anyform
  ```

  ### Cargo (Rust developers)
  ```bash
  cargo install anyform
  ```
  ```

### 5.2 Create CHANGELOG.md
**File:** `CHANGELOG.md`

- [x] Create with 0.4.0 release notes:
  ```markdown
  # Changelog

  ## [0.4.0] - 2025-12-XX

  ### Added
  - Product rename: axum-sea-forms → anyform
  - `anyform init` command for zero-config database setup
  - `anyform serve` with --cors and --no-admin flags
  - Health check endpoint at /health
  - WASM client (anyform-client) for browser-side forms
  - Table prefix changed to `af_`
  - API routes prefixed with `/api/`

  ### Changed
  - CLI binary renamed from `asf` to `anyform`
  - FormsRouter renamed to AnyFormRouter

  ### Migration
  - See docs/migration-0.4.0.md for upgrade guide
  ```

---

## 6. GitHub Secrets Required

Before releasing, ensure these secrets are configured:

- [ ] `DOCKERHUB_USERNAME` - Docker Hub username
- [ ] `DOCKERHUB_TOKEN` - Docker Hub access token
- [ ] `NPM_TOKEN` - npm publish token (for WASM package)
- [ ] GHCR uses `GITHUB_TOKEN` (automatic)

---

## 7. Release Checklist

When ready to release v0.4.0:

1. [ ] Ensure all tests pass
2. [ ] Update version in Cargo.toml files
3. [ ] Update CHANGELOG.md
4. [ ] Create and push git tag: `git tag v0.4.0 && git push --tags`
5. [ ] Verify GitHub Actions workflow runs
6. [ ] Verify release artifacts are uploaded
7. [ ] Verify Docker images are pushed
8. [ ] Test installation methods:
   - [ ] `curl | sh` on Linux
   - [ ] `curl | sh` on macOS
   - [ ] `brew install` on macOS
   - [ ] `scoop install` on Windows
   - [ ] `docker run`
9. [ ] Update Homebrew formula with new checksums
10. [ ] Update Scoop manifest with new checksums

---

## Summary

| Task | Priority | Effort |
|------|----------|--------|
| Update release.yml | High | Medium |
| Update Dockerfile | High | Low |
| Create docker-compose | Medium | Low |
| Create install.sh | High | Low |
| Create Homebrew formula | Medium | Low |
| Create Scoop manifest | Low | Low |
| Update README | High | Low |
| Create CHANGELOG | Medium | Low |

**Estimated total effort**: 2-4 hours

---

*Created: 2025-12-26*
*Last Updated: 2025-12-26*
