import os
import argparse
import qrcode


# Input
parser = argparse.ArgumentParser(description="Generate a QR code.")
parser.add_argument("message", metavar="M", type=str)
parser.add_argument("file_path", metavar="P", type=str)
args = parser.parse_args()

# Constants
MESSAGE = args.message
RESOURCES_DIR = "resources"
OUTPUT_FILE = os.path.join(RESOURCES_DIR, "output", args.file_path)
OUTPUT_DIR = os.path.dirname(OUTPUT_FILE)


if __name__ == "__main__":

    # Create output directory if it doesn't exist
    if os.path.exists(OUTPUT_DIR) is False:
        os.mkdir(OUTPUT_DIR)

    # Generate
    img = qrcode.make(MESSAGE)

    # Save
    img.save(OUTPUT_FILE)
