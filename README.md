# nyxos

[![image](https://img.shields.io/pypi/l/ruff.svg)](https://github.com/astral-sh/ruff/blob/main/LICENSE)
[![CI](https://github.com/geyerandreas/nyxos/actions/workflows/ci.yml/badge.svg)](https://github.com/geyerandreas/nyxos/actions/workflows/ci.yml)

## Roadmap

- [x] User authentication
- [x] JWT token-based authentication
- [ ] oAuth2.0/OpenID Connect integration
- [ ] pypi fall-through, mirroring, and caching
- [ ] posgresql+mysql database integration
- [ ] Amazon S3 integration
- [ ] Google Cloud Storage integration
- [ ] WebUI for managing packages

## Relevant PEPs to be implemented

Packaging PEPs: https://peps.python.org/topic/packaging/

### Core Evolution & Encodings

- PEP 503 – Simple Repository API: The foundational standard defining the base `/simple/`
  HTML format for listing projects and downloadable distribution files.
- PEP 629 – Versioning the Simple API: Introduced the `pypi:repository-version`
  meta tag so installers can understand which API features a registry supports.
- PEP 691 – JSON-based Simple API: Standardized a JSON representation alongside HTML
  using HTTP content negotiation to make package resolution faster and more robust.

### Metadata & Performance Adjustments

- PEP 592 – Yanking Support: Added the ability to mark a specific file as "yanked",
  alerting installers that it has major bugs without removing it entirely and breaking existing locks.
- PEP 658 – Core Metadata in Simple Index: Allowed registries to serve an artifact's
  `.dist-info` core metadata separately, letting clients check package dependencies
  without downloading the entire package wheel.
- PEP 714 – Rename `dist-info-metadata`: Standardized renaming the legacy
  `data-dist-info-metadata` attribute to `data-core-metadata` to patch a critical parser crash in pip.

### API v1.1 through v1.4 Extensions

- PEP 700 – API Version 1.1: Added explicit `versions`, `size`, and `upload-time` fields to the JSON serialization,
  allowing tools like `uv` and `pip` to check release times natively.
- PEP 708 – API Version 1.2 (Provisional): Introduced tracking fields (`tracks`, `alternate-locations`)
  to allow private registries to declare upstream dependencies,
  effectively protecting users against dependency confusion supply-chain attacks.
- PEP 740 – API Version 1.3: Standardized index support for digital attestations.
  Adds data-provenance targets so clients can cryptographically verify who published
  a wheel (e.g., via GitHub Actions Trusted Publishing) instead of relying entirely on vulnerable PGP keys.
- PEP 792 – API Version 1.4: Implemented project status markers (`pypi:project-status`),
  allowing downstream installers to see if a whole project has been `archived` or `quarantined` by index moderators.

### Upcoming Error Handling

- PEP 847 – Problem Details API: Standardizes JSON-based error payloads.
  This eliminates ambiguous plain-text HTTP status strings
  (like generic 401 Unauthorized), replacing them with actionable
  machine-readable error reasons.
