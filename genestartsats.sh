#!/bin/bash

cat > generate_starting_sats.py << 'EOF'
import json

COIN_VALUE = 100_000_000

def get_subsidy(height):
    if height < 101:
        return 1000 * COIN_VALUE      # Epoch 0
    elif height < 1541:
        return 500 * COIN_VALUE       # Epoch 1
    elif height < 2981:
        return 200 * COIN_VALUE       # Epoch 2
    elif height < 5861:
        return 100 * COIN_VALUE       # Epoch 3
    elif height < 262800:
        return 50 * COIN_VALUE        # Epoch 4
    elif height < 394200:
        return 25 * COIN_VALUE        # Epoch 5
    elif height < 657000:
        return int(12.5 * COIN_VALUE) # Epoch 6
    elif height < 1182600:
        return int(6.25 * COIN_VALUE) # Epoch 7
    elif height < 1971000:
        return int(3.125 * COIN_VALUE) # Epoch 8
    elif height < 2759400:
        return int(1.5625 * COIN_VALUE) # Epoch 9
    elif height < 3547800:
        return int(0.78125 * COIN_VALUE) # Epoch 10
    else:
        return int(0.390625 * COIN_VALUE) # Epoch 11 onwards

def calculate_starting_sats():
    starting_sats = [0]  # First sat of epoch 0
    total_sats = 0
    
    # Epoch boundaries
    boundaries = [
        0,      # Epoch 0 start
        101,    # Epoch 1 start
        1541,   # Epoch 2 start
        2981,   # Epoch 3 start
        5861,   # Epoch 4 start
        262800, # Epoch 5 start
        394200, # Epoch 6 start
        657000, # Epoch 7 start
        1182600, # Epoch 8 start
        1971000, # Epoch 9 start
        2759400, # Epoch 10 start
        3547800  # Epoch 11 start
    ]
    
    # Calculate total sats at each epoch boundary
    for i in range(1, len(boundaries)):
        start_height = boundaries[i-1]
        end_height = boundaries[i]
        
        # Add up all subsidies in this epoch
        for height in range(start_height, end_height):
            total_sats += get_subsidy(height)
        
        starting_sats.append(total_sats)
    
    return starting_sats

starting_sats = calculate_starting_sats()
with open('starting_sats.json', 'w') as f:
    json.dump(starting_sats, f, indent=2)
print("Generated starting_sats.json successfully!")
EOF

python3 generate_starting_sats.py
rm generate_starting_sats.py
