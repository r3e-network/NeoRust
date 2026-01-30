from PIL import Image
import sys

input_path = "/home/neo/.gemini/antigravity/brain/5d2b2baa-baf7-46e3-bd71-d12b9c37340f/neo_rust_banner_raw_1769745709210.png"
output_path = "/home/neo/git/neo-rust-sdk/assets/neo_rust_banner.png"

def is_row_empty(img, y, threshold=20):
    width = img.width
    for x in range(width):
        pixel = img.getpixel((x, y))
        # Check alpha channel (index 3)
        if len(pixel) == 4 and pixel[3] > threshold:
            return False
    return True

try:
    img = Image.open(input_path).convert("RGBA")
    width, height = img.size
    
    top = 0
    bottom = height
    
    # Scan from top
    for y in range(height):
        if not is_row_empty(img, y):
            top = y
            break
            
    # Scan from bottom
    for y in range(height - 1, -1, -1):
        if not is_row_empty(img, y):
            bottom = y + 1
            break
            
    print(f"Non-empty content range: y={top} to {bottom}")
    
    if top >= bottom:
        print("Error: content not found (fully transparent?)")
    else:
        # Crop full width, only vertical crop
        crop = img.crop((0, top, width, bottom))
        crop.save(output_path)
        print(f"Saved cropped banner to {output_path}")

except Exception as e:
    print(f"Error: {e}")
