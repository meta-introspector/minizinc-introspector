#!/bin/bash
timeout 10s /data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc /data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/huggingface/word_embeddings_chunk_0.dzn --time-limit 10000 --verbose --verbose-compilation
