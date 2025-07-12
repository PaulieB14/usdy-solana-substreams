#!/usr/bin/env python3
"""
Clean USDY Parquet Streaming
Streams USDY data directly to Parquet format with automatic chunking
"""

import subprocess
import json
import pandas as pd
import os
from datetime import datetime
import time
import signal
import sys

class USDYParquetStreamer:
    def __init__(self):
        # Load environment variables from local.env
        self.load_env_file('local.env')
        
        self.output_dir = "data/usdy_complete_history"
        self.chunk_size = 10000  # blocks per chunk
        self.current_block = 290789141  # Start from first USDY mint
        self.chunk_number = 1  # Start from chunk 1
        self.running = True
        
        # Create output directory
        os.makedirs(self.output_dir, exist_ok=True)
        
        # Setup signal handlers for clean shutdown
        signal.signal(signal.SIGINT, self.signal_handler)
        signal.signal(signal.SIGTERM, self.signal_handler)
    
    def load_env_file(self, filepath):
        """Load environment variables from a file"""
        if os.path.exists(filepath):
            with open(filepath, 'r') as f:
                for line in f:
                    line = line.strip()
                    if line and not line.startswith('#') and '=' in line:
                        key, value = line.split('=', 1)
                        os.environ[key.strip()] = value.strip()
    
    def signal_handler(self, signum, frame):
        print(f"\n🛑 Received signal {signum}, shutting down gracefully...")
        self.running = False
    
    def stream_chunk(self, start_block, end_block, chunk_num):
        """Stream a chunk of blocks and save as Parquet"""
        print(f"📦 Streaming chunk {chunk_num:04d}: blocks {start_block:,} to {end_block:,}")
        
        # Build substreams command
        cmd = [
            "substreams", "run", "substreams.yaml", "map_usdy_events",
            "-e", "mainnet.sol.streamingfast.io:443",
            "--start-block", str(start_block),
            "--stop-block", str(end_block),
            "--output", "jsonl"
        ]
        
        try:
            # Run substreams and capture output (increased timeout for full history)
            result = subprocess.run(cmd, capture_output=True, text=True, timeout=600)
            
            if result.returncode != 0:
                print(f"❌ Substreams error: {result.stderr}")
                return False
            
            # Parse JSONL output
            transactions = []
            for line in result.stdout.strip().split('\n'):
                if line.strip():
                    try:
                        data = json.loads(line)
                        if '@data' in data and 'events' in data['@data']:
                            events = data['@data']['events']
                            for event in events:
                                # Safely convert values
                                block_number = event.get('blockNumber', 0)
                                if isinstance(block_number, str):
                                    block_number = int(block_number) if block_number.isdigit() else 0
                                
                                block_timestamp = event.get('blockTimestamp', 0)
                                if isinstance(block_timestamp, str):
                                    block_timestamp = int(block_timestamp) if block_timestamp.isdigit() else 0
                                
                                instruction_index = event.get('instructionIndex', 0)
                                if isinstance(instruction_index, str):
                                    instruction_index = int(instruction_index) if instruction_index.isdigit() else 0
                                
                                transactions.append({
                                    'signature': event.get('transactionSignature', ''),
                                    'block_number': block_number,
                                    'block_timestamp': datetime.fromtimestamp(block_timestamp),
                                    'block_hash': event.get('blockHash', ''),
                                    'event_type': event.get('eventType', ''),
                                    'instruction_index': instruction_index,
                                    'raw_event': json.dumps(event)
                                })
                    except json.JSONDecodeError:
                        continue
            
            if transactions:
                # Save as Parquet
                df = pd.DataFrame(transactions)
                parquet_file = f"{self.output_dir}/usdy_chunk_{chunk_num:04d}.parquet"
                df.to_parquet(parquet_file, index=False)
                
                print(f"✅ Saved {len(transactions)} events to {parquet_file}")
                return True
            else:
                print(f"⚪ No USDY events found in this range")
                return True
                
        except subprocess.TimeoutExpired:
            print(f"⏰ Timeout streaming chunk {chunk_num}")
            return False
        except Exception as e:
            print(f"❌ Error streaming chunk {chunk_num}: {e}")
            return False
    
    def run(self):
        """Main streaming loop"""
        print("🚀 Starting USDY Parquet Streaming")
        print(f"📁 Output directory: {self.output_dir}")
        print(f"🎯 Starting from block: {self.current_block:,}")
        print(f"📦 Chunk size: {self.chunk_size:,} blocks")
        print("=" * 60)
        
        while self.running:
            start_block = self.current_block
            end_block = start_block + self.chunk_size
            
            success = self.stream_chunk(start_block, end_block, self.chunk_number)
            
            if success:
                self.current_block = end_block + 1
                self.chunk_number += 1
                
                # Save progress
                progress = {
                    'last_block': self.current_block,
                    'next_chunk': self.chunk_number,
                    'timestamp': datetime.now().isoformat()
                }
                with open(f"{self.output_dir}/progress.json", 'w') as f:
                    json.dump(progress, f, indent=2)
                
                print(f"💾 Progress saved: next block {self.current_block:,}")
                
                # Brief pause between chunks
                time.sleep(2)
            else:
                print(f"⚠️  Failed to stream chunk {self.chunk_number}, retrying in 10 seconds...")
                time.sleep(10)
        
        print("\n🎉 Streaming stopped gracefully")
    
    def resume(self):
        """Resume from saved progress"""
        progress_file = f"{self.output_dir}/progress.json"
        if os.path.exists(progress_file):
            try:
                with open(progress_file, 'r') as f:
                    progress = json.load(f)
                self.current_block = progress.get('last_block', self.current_block)
                self.chunk_number = progress.get('next_chunk', self.chunk_number)
                print(f"📂 Resumed from progress: block {self.current_block:,}, chunk {self.chunk_number}")
            except Exception as e:
                print(f"⚠️  Could not load progress: {e}")

def main():
    streamer = USDYParquetStreamer()
    
    # Check for resume
    if len(sys.argv) > 1 and sys.argv[1] == "--resume":
        streamer.resume()
    
    try:
        streamer.run()
    except KeyboardInterrupt:
        print("\n🛑 Interrupted by user")
    except Exception as e:
        print(f"❌ Fatal error: {e}")

if __name__ == "__main__":
    main()
