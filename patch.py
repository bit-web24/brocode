import os
import subprocess
import sys

# Function to format a single Rust file
def format_rust_file(file_path):
    try:
        subprocess.run(["rustfmt", file_path], check=True)
        print(f"Formatted: {file_path}")
    except subprocess.CalledProcessError:
        print(f"Error formatting: {file_path}")
        sys.exit(1)

# Recursively find and format Rust files
def format_rust_files(directory):
    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith(".rs"):
                file_path = os.path.join(root, file)
                format_rust_file(file_path)

# Change the working directory to the root of your project
project_directory = "."
os.chdir(project_directory)

# Format Rust files in the project
format_rust_files(project_directory)

