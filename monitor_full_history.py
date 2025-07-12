#!/usr/bin/env python3
"""
Monitor USDY Complete History Collection
Shows detailed progress and estimates completion time
"""

import json
import os
import glob
import time
from datetime import datetime, timedelta

def monitor_progress():
    print("🌍 USDY Complete History Collection Monitor")
    print("=" * 50)
    
    # Constants
    START_BLOCK = 290789141  # First USDY mint
    CURRENT_SOLANA_BLOCK = 295000000  # Approximate current block
    TOTAL_BLOCKS = CURRENT_SOLANA_BLOCK - START_BLOCK
    CHUNK_SIZE = 10000
    
    # Check progress
    progress_file = "data/usdy_complete_history/progress.json"
    if os.path.exists(progress_file):
        with open(progress_file, 'r') as f:
            progress = json.load(f)
        
        current_block = progress.get('last_block', START_BLOCK)
        chunk_num = progress.get('next_chunk', 1)
        last_update = progress.get('timestamp', '')
        
        # Calculate progress
        blocks_processed = current_block - START_BLOCK
        progress_pct = (blocks_processed / TOTAL_BLOCKS) * 100
        
        print(f"📍 Current Position:")
        print(f"   Block: {current_block:,} / {CURRENT_SOLANA_BLOCK:,}")
        print(f"   Chunk: {chunk_num:,}")
        print(f"   Progress: {progress_pct:.2f}%")
        print(f"   Blocks Processed: {blocks_processed:,} / {TOTAL_BLOCKS:,}")
        
        # Time estimates
        if last_update:
            try:
                last_time = datetime.fromisoformat(last_update.replace('Z', '+00:00'))
                time_elapsed = datetime.now() - last_time
                print(f"   Last Update: {time_elapsed} ago")
            except:
                print(f"   Last Update: {last_update}")
        
        # Estimate completion
        if blocks_processed > 0:
            total_chunks = (TOTAL_BLOCKS // CHUNK_SIZE) + 1
            chunks_remaining = total_chunks - chunk_num
            
            print(f"\n⏱️  Estimates:")
            print(f"   Total Chunks Needed: {total_chunks:,}")
            print(f"   Chunks Remaining: {chunks_remaining:,}")
            
            # Rough time estimate (assuming 30 seconds per chunk average)
            estimated_seconds = chunks_remaining * 30
            estimated_time = timedelta(seconds=estimated_seconds)
            print(f"   Estimated Time Remaining: ~{estimated_time}")
            
            completion_date = datetime.now() + estimated_time
            print(f"   Estimated Completion: {completion_date.strftime('%Y-%m-%d %H:%M')}")
    
    # Check files
    parquet_dir = "data/usdy_complete_history"
    if os.path.exists(parquet_dir):
        parquet_files = glob.glob(f"{parquet_dir}/usdy_chunk_*.parquet")
        
        print(f"\n📦 Data Collection:")
        print(f"   Parquet Files: {len(parquet_files):,} chunks")
        
        if parquet_files:
            # Calculate total size
            total_size = sum(os.path.getsize(f) for f in parquet_files)
            print(f"   Total Size: {total_size / (1024*1024):.1f} MB")
            
            # Show range
            parquet_files.sort()
            first_chunk = os.path.basename(parquet_files[0])
            last_chunk = os.path.basename(parquet_files[-1])
            print(f"   Range: {first_chunk} → {last_chunk}")
            
            # Count total events
            try:
                import pandas as pd
                total_events = 0
                for file in parquet_files[-5:]:  # Check last 5 files for speed
                    try:
                        df = pd.read_parquet(file)
                        total_events += len(df)
                    except:
                        pass
                if total_events > 0:
                    estimated_total = total_events * len(parquet_files) // min(5, len(parquet_files))
                    print(f"   Estimated Total Events: ~{estimated_total:,}")
            except ImportError:
                pass
    
    print(f"\n🎯 Collection Status:")
    print(f"   Target: Complete USDY history from block {START_BLOCK:,}")
    print(f"   Method: 10,000 blocks per Parquet chunk")
    print(f"   Auto-Resume: ✅ Enabled")
    print(f"   Format: Analytics-ready Parquet files")

if __name__ == "__main__":
    monitor_progress()
