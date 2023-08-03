#!/bin/bash

# Check if the number of arguments is correct
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <input_directory> <output_directory>"
    exit 1
fi

input_dir="$1"
output_dir="$2"

# Check if input directory exists
if [ ! -d "$input_dir" ]; then
    echo "Input directory not found: $input_dir"
    exit 1
fi

# Check if output directory exists or create it
if [ ! -d "$output_dir" ]; then
    mkdir -p "$output_dir"
fi

# Loop through all mp3 files in the input directory
for file in "${input_dir}"/*.mp4; do
    # Extract the file name without extension
    filename=$(basename "${file%.mp4}")

    # Execute the ffmpeg command
    ffmpeg -i "${file}" -ar 16000 -ac 1 -c:a pcm_s16le "${output_dir}/${filename}.wav"
done

