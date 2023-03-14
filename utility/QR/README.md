# QR Code Generator

Use `Python` to generate QR codes.

## Quick Start

1. Clone the repository

```shell
git clone https://github.com/RadiantTechLTD/Guides.git
cd Guides
```

2. Change directory to the one containing this file

```shell
cd utility/QR
```

3. Create a virtual environment with `Poetry`:

```shell
poetry install
```

## Generate a QR code

```shell
poetry run python scripts/run.py message file_name
```

### Example

```shell
poetry run python scripts/run.py https://github.com/RadiantTechLtd/Guides/ radiant_tech.png
```
