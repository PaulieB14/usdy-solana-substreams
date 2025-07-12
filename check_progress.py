#!/usr/bin/env python3
"""
Check USDY Parquet streaming progress
"""

import json
import os
import glob
from datetime import datetime

def check_progress():
    print("📊 USDY Parquet Streaming Status")
    print("=" * 40)
    
    # Check progress file
    progress_file = "data/usdy_complete_history/progress.json"
    if os.path.exists(progress_file):
        with open(progress_file, 'r') as f:
            progress = json.load(f)
        
        print(f"📍 Current Position:")
        print(f"   Last Block: {progress.get('last_block', 'N/A'):,}")
        print(f"   Next Chunk: {progress.get('next_chunk', 'N/A')}")
        print(f"   Last Update: {progress.get('timestamp', 'N/A')}")
        
        # Calculate progress percentage
        start_block = 290789141
        current_block = progress.get('last_block', start_block)
        # Approximate current Solana block (this will be outdated quickly)
        estimated_current = 295000000  # Rough estimate
        progress_pct = ((current_block - start_block) / (estimated_current - start_block)) * 100
        print(f"   Progress: ~{progress_pct:.1f}% of history")
    else:
        print("📍 No progress file found")
    
    # Check parquet files
    parquet_dir = "data/usdy_complete_history"
    if os.path.exists(parquet_dir):
        parquet_files = glob.glob(f"{parquet_dir}/usdy_chunk_*.parquet")
        print(f"\n📦 Parquet Files: {len(parquet_files)} chunks")
        
        if parquet_files:
            # Show latest files
            parquet_files.sort()
            print(f"   Latest: {os.path.basename(parquet_files[-1])}")
            
            # Calculate total size
            total_size = sum(os.path.getsize(f) for f in parquet_files)
            print(f"   Total Size: {total_size / (1024*1024):.1f} MB")
            
            # Show block range covered
            first_chunk = os.path.basename(parquet_files[0])
            last_chunk = os.path.basename(parquet_files[-1])
            print(f"   Range: {first_chunk} → {last_chunk}")
    else:
        print("\n📦 No parquet directory found")

if __name__ == "__main__":
    check_progress()
