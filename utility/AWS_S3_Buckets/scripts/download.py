import argparse
import boto3
import os


# Input
parser = argparse.ArgumentParser(description="Download a file.")
parser.add_argument("bucket_name", metavar="B", type=str)
parser.add_argument("bucket_path", metavar="P", type=str)
args = parser.parse_args()

# Constants
BUCKET_NAME = args.bucket_name
BUCKET_PATH = os.path.normpath(args.bucket_path)
FILE_PATH = os.path.basename(BUCKET_PATH)
RESOURCES_DIR = "resources"
OUTPUT_FILE = os.path.join(RESOURCES_DIR, "downloads", FILE_PATH)
OUTPUT_DIR = os.path.dirname(OUTPUT_FILE)


if __name__ == "__main__":

    # Create output directory if it doesn't exist
    if os.path.exists(OUTPUT_DIR) is False:
        os.mkdir(OUTPUT_DIR)

    # Download the file
    s3 = boto3.resource("s3")
    s3.meta.client.download_file(BUCKET_NAME, BUCKET_PATH, OUTPUT_FILE)

    # Print the contents to the console
    print(open(OUTPUT_FILE).read())
