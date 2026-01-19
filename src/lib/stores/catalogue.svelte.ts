/**
 * Catalogue store - manages standard Taguchi arrays
 */
import { invoke } from '@tauri-apps/api/core';
import type { StandardArrayInfo, OAData } from '$lib/types';

// Store state
let _standardArrays = $state<StandardArrayInfo[]>([]);
let _loaded = $state(false);
let _loading = $state(false);
let _error = $state<string | null>(null);

// Filter state
let _filterMinRuns = $state<number | undefined>(undefined);
let _filterMaxRuns = $state<number | undefined>(undefined);
let _filterLevels = $state<number | undefined>(undefined);
let _filterMinFactors = $state<number | undefined>(undefined);

export const catalogueStore = {
  // Getters
  get standardArrays() { return _standardArrays; },
  get loaded() { return _loaded; },
  get loading() { return _loading; },
  get error() { return _error; },

  // Filter getters
  get filterMinRuns() { return _filterMinRuns; },
  get filterMaxRuns() { return _filterMaxRuns; },
  get filterLevels() { return _filterLevels; },
  get filterMinFactors() { return _filterMinFactors; },

  // Computed filtered list
  get filtered(): StandardArrayInfo[] {
    return _standardArrays.filter(arr => {
      if (_filterMinRuns !== undefined && arr.runs < _filterMinRuns) return false;
      if (_filterMaxRuns !== undefined && arr.runs > _filterMaxRuns) return false;
      if (_filterLevels !== undefined && arr.levels !== _filterLevels) return false;
      if (_filterMinFactors !== undefined && arr.factors < _filterMinFactors) return false;
      return true;
    });
  },

  // Filter setters
  setFilter(filter: {
    minRuns?: number;
    maxRuns?: number;
    levels?: number;
    minFactors?: number;
  }) {
    _filterMinRuns = filter.minRuns;
    _filterMaxRuns = filter.maxRuns;
    _filterLevels = filter.levels;
    _filterMinFactors = filter.minFactors;
  },

  clearFilters() {
    _filterMinRuns = undefined;
    _filterMaxRuns = undefined;
    _filterLevels = undefined;
    _filterMinFactors = undefined;
  },

  clearError() {
    _error = null;
  },

  // Tauri commands
  async load(): Promise<void> {
    if (_loaded) return;

    _loading = true;
    _error = null;
    try {
      _standardArrays = await invoke<StandardArrayInfo[]>('list_standard_arrays');
      _loaded = true;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async getArray(name: string): Promise<OAData> {
    _loading = true;
    _error = null;
    try {
      return await invoke<OAData>('get_standard_array', { name });
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async search(filter: {
    minRuns?: number;
    maxRuns?: number;
    levels?: number;
    minFactors?: number;
  }): Promise<StandardArrayInfo[]> {
    try {
      return await invoke<StandardArrayInfo[]>('search_catalogue', {
        minRuns: filter.minRuns,
        maxRuns: filter.maxRuns,
        levels: filter.levels,
        minFactors: filter.minFactors,
      });
    } catch (e) {
      console.error('Search failed:', e);
      return [];
    }
  },
};
