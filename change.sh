#!/bin/bash

echo "Starting replacement of filenames, folder names, and file content for JUNK-20 and JUNK20 variations..."

# Replace all variations of "JUNK-20" and "JUNK20" in folder names
find . -type d -name "*JUNK-20*" -exec bash -c '
for f; do
  new_name="${f//JUNK-20/JUNK-20}"
  echo "Renaming folder: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type d -name "*Junk-20*" -exec bash -c '
for f; do
  new_name="${f//Junk-20/Junk-20}"
  echo "Renaming folder: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type d -name "*junk-20*" -exec bash -c '
for f; do
  new_name="${f//junk-20/junk-20}"
  echo "Renaming folder: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type d -name "*JUNK20*" -exec bash -c '
for f; do
  new_name="${f//JUNK20/JUNK20}"
  echo "Renaming folder: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type d -name "*Junk20*" -exec bash -c '
for f; do
  new_name="${f//Junk20/Junk20}"
  echo "Renaming folder: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type d -name "*junk20*" -exec bash -c '
for f; do
  new_name="${f//junk20/junk20}"
  echo "Renaming folder: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

# Replace all variations of "JUNK-20" and "JUNK20" in file names
find . -type f -name "*JUNK-20*" -exec bash -c '
for f; do
  new_name="${f//JUNK-20/JUNK-20}"
  echo "Renaming file: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type f -name "*Junk-20*" -exec bash -c '
for f; do
  new_name="${f//Junk-20/Junk-20}"
  echo "Renaming file: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type f -name "*junk-20*" -exec bash -c '
for f; do
  new_name="${f//junk-20/junk-20}"
  echo "Renaming file: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type f -name "*JUNK20*" -exec bash -c '
for f; do
  new_name="${f//JUNK20/JUNK20}"
  echo "Renaming file: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type f -name "*Junk20*" -exec bash -c '
for f; do
  new_name="${f//Junk20/Junk20}"
  echo "Renaming file: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

find . -type f -name "*junk20*" -exec bash -c '
for f; do
  new_name="${f//junk20/junk20}"
  echo "Renaming file: $f -> $new_name"
  mv "$f" "$new_name"
done
' _ {} +

# Replace all variations of "JUNK-20" and "JUNK20" in file content
echo "Replacing content in files..."
find . -type f -print0 | xargs -0 sed -i 's/JUNK-20/JUNK-20/g' && echo "Replaced: JUNK-20 -> JUNK-20"
find . -type f -print0 | xargs -0 sed -i 's/Junk-20/Junk-20/g' && echo "Replaced: Junk-20 -> Junk-20"
find . -type f -print0 | xargs -0 sed -i 's/junk-20/junk-20/g' && echo "Replaced: junk-20 -> junk-20"
find . -type f -print0 | xargs -0 sed -i 's/JUNK20/JUNK20/g' && echo "Replaced: JUNK20 -> JUNK20"
find . -type f -print0 | xargs -0 sed -i 's/Junk20/Junk20/g' && echo "Replaced: Junk20 -> Junk20"
find . -type f -print0 | xargs -0 sed -i 's/junk20/junk20/g' && echo "Replaced: junk20 -> junk20"

echo "Replacement process completed."
