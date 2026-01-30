from PIL import Image
import numpy as np
import sys

input_path = "/home/neo/.gemini/antigravity/brain/5d2b2baa-baf7-46e3-bd71-d12b9c37340f/neo_rust_banner_raw_1769745709210.png"
output_path = "/home/neo/git/neo-rust-sdk/assets/neo_rust_banner.png"

try:
    img = Image.open(input_path)
    # Convert to RGBA
    img = img.convert("RGBA")
    
    # Get alpha data
    alpha = np.array(img.split()[-1])
    
    # Find rows with any non-transparent pixels (threshold 10 to ignore dust)
    rows = np.any(alpha > 10, axis=1)
    cols = np.any(alpha > 10, axis=0)
    
    if not np.any(rows):
        print("Error: Image appears fully transparent")
        sys.exit(1)
        
    ymin, ymax = np.where(rows)[0][[0, -1]]
    xmin, xmax = np.where(cols)[0][[0, -1]]
    
    # Add a small padding if possible
    padding = 0
    ymin = max(0, ymin - padding)
    ymax = min(img.height, ymax + padding + 1)
    xmin = max(0, xmin - padding)
    xmax = min(img.width, xmax + padding + 1)
    
    print(f"Content found at: x={xmin}-{xmax}, y={ymin}-{ymax}")
    
    # Crop
    cropped = img.crop((0, ymin, img.width, ymax))
    
    # Save
    cropped.save(output_path)
    print(f"Saved cropped text banner to {output_path}")

except Exception as e:
    print(f"Error processing image: {e}")
