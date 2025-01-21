#!/bin/bash

echo "Starting replacement of filenames and content..."

# Function to handle replacement with proper case preservation
replace_in_files() {
    local old=$1
    local new=$2
    echo "Replacing '$old' with '$new' in file contents..."
    find . -type f -not -path "*/\.*" -exec sed -i "s/$old/$new/g" {} +
}

# Function to rename files and directories
rename_items() {
    local old=$1
    local new=$2
    
    # Find and rename files/directories containing the pattern (case-insensitive)
    find . -depth -name "*$old*" -execdir bash -c '
        old_name="$1"
        new_name="${old_name//$2/$3}"
        if [ "$old_name" != "$new_name" ]; then
            echo "Renaming: $old_name -> $new_name"
            mv "$old_name" "$new_name"
        fi
    ' _ {} "$old" "$new" \;
}

# Replace with different case variations
replace_in_files "june" "june"
replace_in_files "June" "June"
replace_in_files "JUNE" "JUNE"

# Rename files and directories
rename_items "june" "june"
rename_items "June" "June"
rename_items "JUNE" "JUNE"

echo "Replacement process completed."
