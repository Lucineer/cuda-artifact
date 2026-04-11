# cuda-artifact

Post-human artifact layer — deliberation output becomes adaptive executable programs

Part of the Cocapn fleet — a Lucineer vessel component.

## What It Does

### Key Types

- `ProvenanceEntry` — core data structure
- `Artifact` — core data structure
- `AdaptationPolicy` — core data structure
- `ArtifactCheckpoint` — core data structure
- `ArtifactRegistry` — core data structure

## Quick Start

```bash
# Clone
git clone https://github.com/Lucineer/cuda-artifact.git
cd cuda-artifact

# Build
cargo build

# Run tests
cargo test
```

## Usage

```rust
use cuda_artifact::*;

// See src/lib.rs for full API
// 5 unit tests included
```

### Available Implementations

- `Default for AdaptationPolicy` — see source for methods
- `Artifact` — see source for methods
- `ArtifactRegistry` — see source for methods

## Testing

```bash
cargo test
```

5 unit tests covering core functionality.

## Architecture

This crate is part of the **Cocapn Fleet** — a git-native multi-agent ecosystem.

- **Category**: other
- **Language**: Rust
- **Dependencies**: See `Cargo.toml`
- **Status**: Active development

## Related Crates


## Fleet Position

```
Casey (Captain)
├── JetsonClaw1 (Lucineer realm — hardware, low-level systems, fleet infrastructure)
├── Oracle1 (SuperInstance — lighthouse, architecture, consensus)
└── Babel (SuperInstance — multilingual scout)
```

## Contributing

This is a fleet vessel component. Fork it, improve it, push a bottle to `message-in-a-bottle/for-jetsonclaw1/`.

## License

MIT

---

*Built by JetsonClaw1 — part of the Cocapn fleet*
*See [cocapn-fleet-readme](https://github.com/Lucineer/cocapn-fleet-readme) for the full fleet roadmap*
