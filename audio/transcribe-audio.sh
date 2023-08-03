#!/bin/bash
echo "Must be run from within audio folder. And assumes whisper.cpp built in a directory adjacent to this chat bot."
./../../whisper.cpp/main -p 3 -otxt -m ./../../whisper.cpp/models/ggml-large.bin -f ./*.wav