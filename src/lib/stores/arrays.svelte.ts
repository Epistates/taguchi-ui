/**
 * Array store - manages orthogonal arrays in the application
 * Uses tauri-plugin-svelte RuneStore for Svelte 5 runes + persistence
 */
import { invoke } from '@tauri-apps/api/core';
import { RuneStore } from 'tauri-plugin-svelte';
import type { OAData, BuildRequest, ConstructionOption, ValidationResult } from '$lib/types';

// Serializable state for persistence (requires index signature for State constraint)
interface ArrayStoreState {
  [key: string]: unknown;
  arrays: Record<string, OAData>;
}

// Create persistent rune store
const persistentStore = new RuneStore<ArrayStoreState>('array-store', {
  arrays: {},
});

// Store state
let _arrays = $state<Map<string, OAData>>(new Map());
let _currentId = $state<string | null>(null);
let _loading = $state(false);
let _error = $state<string | null>(null);
let _initialized = $state(false);

// Convert Record to Map
function recordToMap<T>(record: Record<string, T>): Map<string, T> {
  return new Map(Object.entries(record));
}

// Convert Map to Record
function mapToRecord<T>(map: Map<string, T>): Record<string, T> {
  return Object.fromEntries(map.entries());
}

// Save current state to persistent store
async function saveState() {
  // Update the reactive state object directly
  persistentStore.state.arrays = mapToRecord(_arrays);
  // Trigger save to disk
  await persistentStore.save();
}

// Load state from persistent store
async function loadState() {
  try {
    await persistentStore.start();
    // After start(), state is populated from disk
    const state = persistentStore.state;
    if (state) {
      _arrays = recordToMap(state.arrays || {});
    }
    _initialized = true;
  } catch (e) {
    console.error('Failed to load array store:', e);
    _initialized = true;
  }
}

// Initialize on module load
loadState();

export const arrayStore = {
  // Getters
  get items() { return _arrays; },
  get current() { return _currentId ? _arrays.get(_currentId) ?? null : null; },
  get currentId() { return _currentId; },
  get loading() { return _loading; },
  get error() { return _error; },
  get list() { return Array.from(_arrays.values()); },
  get initialized() { return _initialized; },

  // Initialization
  async initialize() {
    if (!_initialized) {
      await loadState();
    }
  },

  // Actions
  setCurrent(id: string | null) {
    _currentId = id;
  },

  get(id: string): OAData | undefined {
    return _arrays.get(id);
  },

  async add(oa: OAData) {
    _arrays.set(oa.id, oa);
    await saveState();
  },

  async remove(id: string) {
    _arrays.delete(id);
    if (_currentId === id) {
      _currentId = null;
    }
    await saveState();
  },

  // Update array metadata
  async updateMetadata(id: string, updates: { name?: string; notes?: string }): Promise<OAData | null> {
    const oa = _arrays.get(id);
    if (oa) {
      const updated: OAData = {
        ...oa,
        metadata: {
          ...oa.metadata,
          ...(updates.name !== undefined && { name: updates.name }),
          ...(updates.notes !== undefined && { notes: updates.notes }),
        },
      };
      _arrays.set(id, updated);
      await saveState();
      return updated;
    }
    return null;
  },

  // Duplicate an array with a new ID
  async duplicate(id: string, newName?: string): Promise<OAData | null> {
    const oa = _arrays.get(id);
    if (oa) {
      const newId = crypto.randomUUID();
      const duplicated: OAData = {
        ...oa,
        id: newId,
        metadata: {
          ...oa.metadata,
          name: newName || `${oa.metadata.name || 'Array'} (Copy)`,
          createdAt: new Date().toISOString(),
        },
      };
      _arrays.set(newId, duplicated);
      await saveState();
      return duplicated;
    }
    return null;
  },

  async clear() {
    _arrays.clear();
    _currentId = null;
    await saveState();
  },

  clearError() {
    _error = null;
  },

  // Tauri commands
  async build(request: BuildRequest): Promise<OAData> {
    _loading = true;
    _error = null;
    try {
      const result = await invoke<OAData>('build_oa', { request });
      _arrays.set(result.id, result);
      _currentId = result.id;
      await saveState();
      return result;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async getAvailableConstructions(levels: number, strength: number): Promise<ConstructionOption[]> {
    try {
      return await invoke<ConstructionOption[]>('get_available_constructions', {
        levels,
        strength,
      });
    } catch (e) {
      console.error('Failed to get constructions:', e);
      return [];
    }
  },

  async validateParams(request: BuildRequest): Promise<ValidationResult> {
    try {
      return await invoke<ValidationResult>('validate_build_params', { request });
    } catch (e) {
      return {
        valid: false,
        errors: [String(e)],
        warnings: [],
        suggestions: [],
      };
    }
  },
};
