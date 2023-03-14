import argparse
import boto3
import os

# Input
parser = argparse.ArgumentParser(description="Upload a file.")
parser.add_argument("bucket_name", metavar="B", type=str)
parser.add_argument("file_path", metavar="P", type=str)
args = parser.parse_args()

# Constants
BUCKET_NAME = args.bucket_name
FILE_NAME = os.path.normpath(args.file_path)
RESOURCES_DIR = "resources"
INPUT_DIR = os.path.join(RESOURCES_DIR, "uploads")
INPUT_FILE = os.path.join(INPUT_DIR, FILE_NAME)


if __name__ == "__main__":

    # Create output directory if it doesn't exist
    if os.path.exists(INPUT_DIR) is False:
        os.mkdir(INPUT_DIR)

    # Upload the file
    s3 = boto3.resource("s3")
    s3.meta.client.upload_file(INPUT_FILE, BUCKET_NAME, FILE_NAME)
