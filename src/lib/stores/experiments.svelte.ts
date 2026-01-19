/**
 * Experiments store - manages experiment persistence and import/export
 */
import { invoke } from '@tauri-apps/api/core';
import { save, open } from '@tauri-apps/plugin-dialog';
import type { OAData, ImportValidation } from '$lib/types';

// Store state
let _recentExports = $state<string[]>([]);
let _loading = $state(false);
let _error = $state<string | null>(null);

export const experimentsStore = {
  // Getters
  get recentExports() { return _recentExports; },
  get loading() { return _loading; },
  get error() { return _error; },

  clearError() {
    _error = null;
  },

  // Export functions
  async exportCSV(data: OAData): Promise<string | null> {
    _loading = true;
    _error = null;
    try {
      const path = await save({
        filters: [{ name: 'CSV', extensions: ['csv'] }],
        defaultPath: `${data.metadata.name || 'array'}.csv`,
      });

      if (path) {
        await invoke('export_csv', { data, path });
        _recentExports = [path, ..._recentExports.slice(0, 9)];
        return path;
      }
      return null;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async exportJSON(data: OAData): Promise<string | null> {
    _loading = true;
    _error = null;
    try {
      const path = await save({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        defaultPath: `${data.metadata.name || 'array'}.json`,
      });

      if (path) {
        await invoke('export_json', { data, path });
        _recentExports = [path, ..._recentExports.slice(0, 9)];
        return path;
      }
      return null;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async exportLaTeX(data: OAData): Promise<string> {
    _loading = true;
    _error = null;
    try {
      return await invoke<string>('export_latex', { data });
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  // Import functions
  async importCSV(): Promise<{ data: number[][]; path: string } | null> {
    _loading = true;
    _error = null;
    try {
      const path = await open({
        filters: [{ name: 'CSV', extensions: ['csv'] }],
        multiple: false,
      });

      if (path && typeof path === 'string') {
        const data = await invoke<number[][]>('import_csv', { path });
        return { data, path };
      }
      return null;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async importJSON(): Promise<OAData | null> {
    _loading = true;
    _error = null;
    try {
      const path = await open({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        multiple: false,
      });

      if (path && typeof path === 'string') {
        return await invoke<OAData>('import_json', { path });
      }
      return null;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async validateImport(data: number[][]): Promise<ImportValidation> {
    _loading = true;
    _error = null;
    try {
      return await invoke<ImportValidation>('validate_import', { data });
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  // Copy LaTeX to clipboard
  async copyLaTeXToClipboard(data: OAData): Promise<void> {
    const latex = await this.exportLaTeX(data);
    await navigator.clipboard.writeText(latex);
  },
};
