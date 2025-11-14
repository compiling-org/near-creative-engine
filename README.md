# NEAR Creative Engine - Fractal Studio

This repository contains the NEAR Foundation grant implementation for real-time fractal generation with emotional computing.

## Project Overview

We propose developing a Rust-to-WASM compilation module that enables direct deployment of our NUWE (Neuro-Unified Wave Environment) fractal shader engine as a NEAR Component (dApp) in the browser. This will create user-owned, real-time creative sessions that run entirely on-chain, democratizing access to high-performance creative tools as public goods.

## Features

- **Real-time Fractal Generation**: Interactive Mandelbrot, Julia, and Burning Ship fractals
- **Emotional Computing Integration**: Visual parameters modulated by emotional state
- **NEAR Blockchain Integration**: Sessions saved and owned on NEAR
- **Browser-Native Performance**: WASM-compiled engine for desktop-quality performance
- **Practical Emotional Input**: Multiple input methods including camera-based facial expression analysis

## Getting Started

### Prerequisites

- Rust and Cargo
- Node.js and npm
- NEAR CLI
- wasm-pack

### Installation

```bash
# Install CLI tools
./scripts/install-cli-tools.sh

# Build the project
./build-near-grant.sh
```

### Building

```bash
# Build NEAR smart contracts
cd src/near-wasm
cargo build --target wasm32-unknown-unknown --release

# Build WASM for browser
wasm-pack build --target web --out-dir ../../test-website/wasm
```

### Deployment

1. Deploy contracts to NEAR testnet
2. Update contract IDs in test-website configuration
3. Serve test-website on a web server

## Practical Emotional Input Methods

Our system implements multiple practical approaches for emotional input that work without specialized hardware:

1. **Manual Slider Controls** (Primary Method)
   - Valence (Negative ↔ Positive) slider
   - Arousal (Calm ↔ Excited) slider
   - Dominance (Submissive ↔ Dominant) slider
   - Works on any device with a browser

2. **Camera-Based Facial Expression Analysis** (Enhancement)
   - MediaPipe FaceMesh for facial landmark detection
   - Real-time mapping of facial expressions to emotional states
   - Works with any standard webcam (built into most devices)
   - Local processing - no data leaves the user's device

3. **Keyboard/Mouse Interaction Tracking** (Passive Input)
   - Typing pattern analysis for arousal inference
   - Mouse movement dynamics for emotional state detection
   - Subtle emotional updates based on interaction behavior

4. **Voice Tone Analysis** (Future Enhancement)
   - Audio input processing for emotional inference
   - Works with standard microphones
   - Local processing for privacy

5. **EEG/BMI Integration** (Future Research Enhancement)
   - Compatible with consumer EEG devices (OpenBCI, Muse, etc.)
   - Advanced biometric emotional state detection
   - Research-grade precision for specialized applications

## Directory Structure

```
├── src/
│   ├── near-wasm/          # NEAR smart contracts and WASM engine
│   └── rust-client/        # Core Rust library (shared dependency)
├── test-website/           # Browser-based frontend
├── scripts/                # Utility scripts
├── build-near-grant.sh     # Build script
└── README.md              # This file
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

- **Website**: https://compiling-org.netlify.app
- **GitHub**: https://github.com/compiling-org
- **Email**: kapil.bambardekar@gmail.com, vdmo@gmail.com
