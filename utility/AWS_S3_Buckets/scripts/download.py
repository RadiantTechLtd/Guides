import boto3
import os

# Set these constants
BUCKET_NAME = "some_bucket_that_already_exists"
FILENAME = "data.csv"
RESOURCES_DIR = "resources"

# Derived constants
OUTPUT_DIR = os.path.join(RESOURCES_DIR, "downloads")
OUTPUT_FILE = os.path.join(OUTPUT_DIR, FILENAME)

# Create output directory if it doesn't exist
if os.path.exists(OUTPUT_DIR) is False:
    os.mkdir(OUTPUT_DIR)

# Download the file
s3 = boto3.resource("s3")
s3.meta.client.download_file(BUCKET_NAME, FILENAME, OUTPUT_FILE)

# Print the contents to the console
print(open(OUTPUT_FILE).read())
