import boto3
import os

# Set these constants
BUCKET_NAME = "some_bucket_that_already_exists"
FILENAME = "data.csv"
RESOURCES_DIR = "resources"

# Derived constants
INPUT_DIR = os.path.join(RESOURCES_DIR, "uploads")
INPUT_FILE = os.path.join(INPUT_DIR, FILENAME)

# Create output directory if it doesn't exist
if os.path.exists(INPUT_DIR) is False:
    os.mkdir(INPUT_DIR)

# Upload the file
s3 = boto3.resource("s3")
s3.meta.client.upload_file(INPUT_FILE, BUCKET_NAME, INPUT_FILE)
