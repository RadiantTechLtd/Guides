# Test Upload

How to upload and download files to and from AWS S3 buckets.
Note we expect that the bucket already exists.
We also expect that the AWS credentials are already set up.

## Quick Start

1. Clone the repository

```shell
git clone https://github.com/RadiantTechLTD/Guides.git
cd Guides
```

2. Change directory to the one containing this file

```shell
cd utility/AWS_S3_Buckets
```

3. Create a virtual environment with `Poetry`:

```shell
poetry install
```

## Upload a file

```shell
poetry run python scripts/upload.py <bucket_name> <file_path>
```

## Download a file

```shell
poetry run python scripts/download.py <bucket_name> <bucket_path>
```
