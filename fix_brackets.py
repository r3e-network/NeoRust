#!/usr/bin/env python3

with open('neo-rust-gui/src/app.rs', 'r') as f:
    lines = f.readlines()

# Find render_nft_view function
start_line = None
for i, line in enumerate(lines):
    if 'fn render_nft_view' in line:
        start_line = i
        break

if start_line:
    print(f"render_nft_view starts at line {start_line + 1}")
    
    open_braces = 0
    open_parens = 0
    
    for i in range(start_line, len(lines)):
        line = lines[i]
        open_braces += line.count('{') - line.count('}')
        open_parens += line.count('(') - line.count(')')
        
        if i > start_line and open_braces == 0:
            print(f"Function should end at line {i + 1}: {line.strip()}")
            break
            
        if i == start_line + 100:  # Check first 100 lines
            print(f"After 100 lines: braces={open_braces}, parens={open_parens}")
            break 