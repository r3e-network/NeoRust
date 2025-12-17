# Changelog

All notable changes to the Neo GUI project will be documented in this file.

## [0.5.4] - 2025-12-17

### Added
- **Reusable Modal System**: Introduced a new directory structure for modal components (`src/components/modals/`) to promote code reusability and cleaner architecture.
  - `CreateWalletModal`: A dedicated modal for creating new wallets with name and password validation.
  - `ImportWalletModal`: A new modal for importing existing wallets using private keys (WIF/Hex).
- **Backend Integration**: Fully connected the `Dashboard` and `Wallet` pages to the Rust backend using Tauri's `invoke` command.
  - Real-time transaction history fetching.
  - Wallet creation and importation now trigger actual backend commands.
- **Enhanced UI/UX**:
  - Implemented "Beautifully Aligned" design principles across key pages (`Dashboard`, `Wallet`, `Settings`).
  - Added smooth page transitions and interactive element animations using `framer-motion`.
  - integrated `Recharts` for responsive data visualization of price history and portfolio distribution.
  - Unified styling with a consistent Neo Green theme using Tailwind CSS.

### Changed
- Refactored `Dashboard.tsx` to use the new `CreateWalletModal` and display real store data instead of static placeholders.
- Refactored `Wallet.tsx` to use both `CreateWalletModal` and `ImportWalletModal`, removing the previous inline modal implementations.
- Updated `Settings.tsx` with a polished tabbed interface and persisted user preferences.
- Improved error handling and notification feedback for wallet operations.

### Fixed
- Fixed TypeScript errors related to missing imports and type definitions in `Wallet.tsx`.
- Resolved state management inconsistencies in the `appStore` to better reflect backend state.
