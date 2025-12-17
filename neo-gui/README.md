# Neo GUI Frontend

A modern React-based GUI frontend for the NeoRust blockchain toolkit, built with Vite, TypeScript, and Tailwind CSS.

## Features

- 🚀 **Modern Tech Stack**: Built with React 18, TypeScript, and Vite for blazing fast performance.
- 🎨 **Beautiful UI/UX**: Polished design with Tailwind CSS, featuring consistent "Neo Green" theming and "beautifully aligned" layouts.
- ⚡ **Desktop Integration**: Powered by Tauri for native desktop app functionality with Rust backend integration.
- 🔐 **Secure Wallet Management**:
  - Create and import wallets (WIF/Hex support).
  - Secure password handling and validation.
  - Real-time balance and transaction history tracking.
- 📊 **Rich Data Visualization**: Interactive charts for price history and portfolio distribution using Recharts.
- 🎭 **Smooth Animations**: Enhanced user experience with fluid page transitions and component animations via Framer Motion.
- 📱 **Responsive Design**: Fully responsive layouts adaptable to different screen sizes.
- 🧩 **Modular Architecture**: Reusable component system with dedicated modals and optimized state management using Zustand.

## Prerequisites

- Node.js 18.x or higher
- npm 10.x or higher
- Rust (for Tauri desktop app functionality)

### System Dependencies (Linux only)

For building the desktop app on Linux, you'll need GTK development libraries:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
```

**Fedora:**
```bash
sudo dnf install gtk3-devel webkit2gtk4.0-devel libappindicator-gtk3-devel librsvg2-devel
```

**Arch Linux:**
```bash
sudo pacman -S gtk3 webkit2gtk libappindicator-gtk3 librsvg
```

## Quick Start

1. **Install dependencies:**
   ```bash
   npm install
   ```

2. **Start development server:**
   ```bash
   npm run dev
   ```

3. **Open your browser:**
   Navigate to `http://localhost:1420/`

## Available Scripts

- `npm run dev` - Start development server
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run tauri` - Run Tauri commands
- `npm run clean` - Remove node_modules and package-lock.json
- `npm run fresh-install` - Clean install (removes cache and reinstalls)
- `npm run fix-deps` - Fix dependency issues (installs util-deprecate)

## Development Workflow

1. **Start the development server:**
   ```bash
   npm run dev
   ```

2. **Make your changes** to the React components in the `src/` directory

3. **Build for production:**
   ```bash
   npm run build
   ```

## Project Structure

```
neo-gui/
├── src/                    # React source code
├── assets/                 # Static assets
├── index.html             # HTML template
├── package.json           # Dependencies and scripts
├── postcss.config.js      # PostCSS configuration
├── tailwind.config.js     # Tailwind CSS configuration
├── tsconfig.json          # TypeScript configuration
├── vite.config.ts         # Vite configuration
└── tauri.conf.json        # Tauri configuration
```

## Technologies Used

- **Frontend Framework:** React 18 with TypeScript
- **Build Tool:** Vite
- **Styling:** Tailwind CSS, Headless UI
- **Desktop App:** Tauri
- **UI Components:** Heroicons, Headless UI
- **Charts:** Recharts
- **Animations:** Framer Motion
- **State Management:** Zustand
- **Routing:** React Router
- **Utilities:** clsx, date-fns

## Troubleshooting

If you encounter any issues during development, please refer to the [TROUBLESHOOTING.md](./TROUBLESHOOTING.md) guide.

**Common Issues:**
- [Cannot find module 'util-deprecate'](./TROUBLESHOOTING.md#cannot-find-module-util-deprecate-error)

## Contributing

1. Make sure all tests pass
2. Follow the existing code style
3. Update documentation as needed
4. Test your changes thoroughly

## License

This project is part of the NeoRust toolkit. See the main project LICENSE for details. 