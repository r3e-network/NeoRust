# Modal Components

This directory contains reusable modal components used throughout the Neo GUI application.

## Components

### `CreateWalletModal`

A modal dialog for creating a new wallet.

**Props:**
- `isOpen` (boolean): Controls the visibility of the modal.
- `onClose` (function): Callback function to close the modal.
- `onSubmit` (function): Async callback function to handle wallet creation. Receives `name` and `password`.
- `loading` (boolean): Indicates if the creation process is in progress.

**Usage:**
```tsx
import CreateWalletModal from '../components/modals/CreateWalletModal';

<CreateWalletModal
  isOpen={showCreateModal}
  onClose={() => setShowCreateModal(false)}
  onSubmit={handleCreateWallet}
  loading={loading}
/>
```

### `ImportWalletModal`

A modal dialog for importing an existing wallet using a private key.

**Props:**
- `isOpen` (boolean): Controls the visibility of the modal.
- `onClose` (function): Callback function to close the modal.
- `onSubmit` (function): Async callback function to handle wallet import. Receives `privateKey`, `name`, and `password`.
- `loading` (boolean): Indicates if the import process is in progress.

**Usage:**
```tsx
import ImportWalletModal from '../components/modals/ImportWalletModal';

<ImportWalletModal
  isOpen={showImportModal}
  onClose={() => setShowImportModal(false)}
  onSubmit={handleImportWallet}
  loading={loading}
/>
```
