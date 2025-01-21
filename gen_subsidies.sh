#!/bin/bash

cat > generate_subsidies.py << 'EOF'
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

epochs = {str(height): get_subsidy(height) for height in range(4000000)}
with open('subsidies.json', 'w') as f:
    json.dump({"epochs": epochs}, f, indent=2)
print("Generated subsidies.json successfully!")
EOF

python3 generate_subsidies.py
rm generate_subsidies.py
