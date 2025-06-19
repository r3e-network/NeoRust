# Neo GUI Frontend Troubleshooting Guide

## Common Issues and Solutions

### "Cannot find module 'util-deprecate'" Error

**Problem:** When running `npm run dev`, the build fails with the error:
```
Cannot find module 'util-deprecate'
```

This error occurs in the PostCSS/Tailwind CSS pipeline during the build process.

**Environment:** 
- Affects Node.js versions 18.x and 22.x
- Common on macOS ARM64 systems
- Intermittent issue that may not affect all environments

**Root Cause:**
The `util-deprecate` module is a transitive dependency of `postcss-selector-parser` (used by Tailwind CSS). Node.js module resolution sometimes fails to locate this dependency, even when it's installed in `node_modules`.

**Solutions:**

#### Quick Fix (Immediate Resolution)
```bash
# Navigate to the neo-gui directory
cd neo-gui

# Install util-deprecate explicitly
npm install util-deprecate
```

#### Permanent Fix (Recommended)
The `package.json` has been updated to include `util-deprecate` as an explicit dependency to prevent this issue.

#### Complete Clean Install
If the issue persists:
```bash
# Remove existing installations
rm -rf node_modules package-lock.json

# Clear npm cache
npm cache clean --force

# Reinstall dependencies
npm install

# Run development server
npm run dev
```

#### Alternative: Use Yarn
If npm continues to have issues:
```bash
# Install yarn if not already installed
npm install -g yarn

# Remove npm artifacts
rm -rf node_modules package-lock.json

# Install with yarn
yarn install

# Run development server
yarn dev
```

### Additional Notes

- This issue is related to Node.js module resolution and may affect different environments differently
- The explicit dependency addition should resolve the issue permanently
- If you encounter this issue again, please update this documentation with your environment details

### Environment Compatibility

**Tested Working Configurations:**
- Node.js v23.9.0 + npm v10.9.2 (macOS ARM64)

**Known Issues:**
- Node.js v18.20.8 + npm v10.8.2 (macOS ARM64) - requires explicit util-deprecate installation
- Node.js v22.13.1 + npm v10.8.2 (macOS ARM64) - requires explicit util-deprecate installation 