/**
 * DOE Store - manages experiment configurations, results, and analysis
 * Uses tauri-plugin-svelte RuneStore for Svelte 5 runes + persistence
 */
import { invoke } from '@tauri-apps/api/core';
import { RuneStore } from 'tauri-plugin-svelte';
import type {
  DOEConfig,
  ExperimentFactor,
  ResponseVariable,
  ExperimentResults,
  Measurement,
  DOEAnalysis,
  DOEAnalysisRequest,
  AnalysisSettings,
} from '$lib/types';
import { DEFAULT_ANALYSIS_SETTINGS } from '$lib/types';

// Serializable state for persistence (requires index signature for State constraint)
interface DOEStoreState {
  [key: string]: unknown;
  configs: Record<string, DOEConfig>;
  results: Record<string, ExperimentResults>;
  analyses: Record<string, DOEAnalysis>;
}

// Create persistent rune store
const persistentStore = new RuneStore<DOEStoreState>('doe-store', {
  configs: {},
  results: {},
  analyses: {},
});

// Local reactive state (synced with persistent store)
let _configs = $state<Map<string, DOEConfig>>(new Map());
let _results = $state<Map<string, ExperimentResults>>(new Map());
let _analyses = $state<Map<string, DOEAnalysis>>(new Map());
let _currentConfigId = $state<string | null>(null);
let _loading = $state(false);
let _error = $state<string | null>(null);
let _initialized = $state(false);

// Draft state for configuration wizard (not persisted)
let _draftConfig = $state<Partial<DOEConfig> | null>(null);

// Convert Record to Map
function recordToMap<T>(record: Record<string, T>): Map<string, T> {
  return new Map(Object.entries(record));
}

// Convert Map to Record
function mapToRecord<T>(map: Map<string, T>): Record<string, T> {
  return Object.fromEntries(map.entries());
}

// Trigger reactivity by reassigning Maps (Svelte 5 doesn't track Map mutations)
function triggerReactivity() {
  _configs = new Map(_configs);
  _results = new Map(_results);
  _analyses = new Map(_analyses);
}

// Save current state to persistent store
async function saveState() {
  // Trigger reactivity before saving
  triggerReactivity();
  // Update the reactive state object directly
  persistentStore.state.configs = mapToRecord(_configs);
  persistentStore.state.results = mapToRecord(_results);
  persistentStore.state.analyses = mapToRecord(_analyses);
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
      _configs = recordToMap(state.configs || {});
      _results = recordToMap(state.results || {});
      _analyses = recordToMap(state.analyses || {});
    }
    _initialized = true;
  } catch (e) {
    console.error('Failed to load DOE store:', e);
    _initialized = true;
  }
}

// Initialize on module load
loadState();

export const doeStore = {
  // ========================================
  // Getters
  // ========================================
  get configs() { return _configs; },
  get results() { return _results; },
  get analyses() { return _analyses; },
  get currentConfigId() { return _currentConfigId; },
  get currentConfig() { return _currentConfigId ? _configs.get(_currentConfigId) ?? null : null; },
  get currentResults() { return _currentConfigId ? _results.get(_currentConfigId) ?? null : null; },
  get currentAnalysis() { return _currentConfigId ? _analyses.get(_currentConfigId) ?? null : null; },
  get loading() { return _loading; },
  get error() { return _error; },
  get draftConfig() { return _draftConfig; },
  get configList() { return Array.from(_configs.values()); },
  get initialized() { return _initialized; },

  // ========================================
  // Initialization
  // ========================================

  async initialize() {
    if (!_initialized) {
      await loadState();
    }
  },

  // ========================================
  // Config CRUD
  // ========================================

  setCurrent(id: string | null) {
    _currentConfigId = id;
  },

  getConfig(id: string): DOEConfig | undefined {
    return _configs.get(id);
  },

  /**
   * Create a new experiment configuration
   */
  async createConfig(
    arrayId: string,
    name: string,
    factors: ExperimentFactor[],
    responses: ResponseVariable[],
    replicates: number,
    analysisSettings: AnalysisSettings = DEFAULT_ANALYSIS_SETTINGS
  ): Promise<DOEConfig> {
    const now = new Date().toISOString();
    const config: DOEConfig = {
      id: crypto.randomUUID(),
      name,
      arrayId,
      factors,
      responses,
      replicates,
      analysisSettings,
      createdAt: now,
      updatedAt: now,
    };
    _configs.set(config.id, config);
    _currentConfigId = config.id;

    // Initialize empty results
    this.initializeResults(config.id, config);

    await saveState();
    return config;
  },

  /**
   * Update an existing configuration
   */
  async updateConfig(id: string, updates: Partial<Omit<DOEConfig, 'id' | 'createdAt'>>): Promise<DOEConfig | null> {
    const config = _configs.get(id);
    if (!config) return null;

    const updated: DOEConfig = {
      ...config,
      ...updates,
      updatedAt: new Date().toISOString(),
    };
    _configs.set(id, updated);
    await saveState();
    return updated;
  },

  /**
   * Delete a configuration and its associated results/analysis
   */
  async deleteConfig(id: string) {
    _configs.delete(id);
    _results.delete(id);
    _analyses.delete(id);
    if (_currentConfigId === id) {
      _currentConfigId = null;
    }
    await saveState(); // Also triggers reactivity
  },

  // ========================================
  // Draft Config (for wizard flow - not persisted)
  // ========================================

  /**
   * Start a new draft configuration
   */
  startDraft(arrayId: string, numFactors: number, numLevels: number[]) {
    // Create default factors based on array dimensions
    const factors: ExperimentFactor[] = Array.from({ length: numFactors }, (_, i) => ({
      id: crypto.randomUUID(),
      name: `Factor ${i + 1}`,
      unit: undefined,
      levelValues: Array.from({ length: numLevels[i] ?? numLevels[0] }, (_, j) => j),
    }));

    // Create default response
    const responses: ResponseVariable[] = [{
      id: crypto.randomUUID(),
      name: 'Response',
      unit: undefined,
      optimizationType: 'larger-is-better',
      targetValue: undefined,
    }];

    _draftConfig = {
      arrayId,
      name: 'New Experiment',
      factors,
      responses,
      replicates: 1,
      analysisSettings: { ...DEFAULT_ANALYSIS_SETTINGS },
    };
  },

  /**
   * Update the draft configuration
   */
  updateDraft(updates: Partial<DOEConfig>) {
    if (_draftConfig) {
      _draftConfig = { ..._draftConfig, ...updates };
    }
  },

  /**
   * Update a factor in the draft
   */
  updateDraftFactor(factorId: string, updates: Partial<ExperimentFactor>) {
    if (_draftConfig?.factors) {
      _draftConfig = {
        ..._draftConfig,
        factors: _draftConfig.factors.map(f =>
          f.id === factorId ? { ...f, ...updates } : f
        ),
      };
    }
  },

  /**
   * Update a response in the draft
   */
  updateDraftResponse(responseId: string, updates: Partial<ResponseVariable>) {
    if (_draftConfig?.responses) {
      _draftConfig = {
        ..._draftConfig,
        responses: _draftConfig.responses.map(r =>
          r.id === responseId ? { ...r, ...updates } : r
        ),
      };
    }
  },

  /**
   * Finalize the draft into a real config
   */
  async finalizeDraft(): Promise<DOEConfig | null> {
    if (!_draftConfig || !_draftConfig.arrayId || !_draftConfig.name ||
        !_draftConfig.factors || !_draftConfig.responses) {
      return null;
    }

    const config = await this.createConfig(
      _draftConfig.arrayId,
      _draftConfig.name,
      _draftConfig.factors,
      _draftConfig.responses,
      _draftConfig.replicates ?? 1,
      _draftConfig.analysisSettings ?? DEFAULT_ANALYSIS_SETTINGS
    );

    _draftConfig = null;
    return config;
  },

  /**
   * Cancel the draft
   */
  cancelDraft() {
    _draftConfig = null;
  },

  // ========================================
  // Results Management
  // ========================================

  /**
   * Initialize empty results for a configuration
   */
  initializeResults(configId: string, config: DOEConfig) {
    _results.set(configId, {
      configId,
      measurements: [],
      completedAt: undefined,
    });
  },

  /**
   * Set up results with the correct number of runs
   * Preserves existing measurement values when re-initializing
   */
  async setupResultsForArray(configId: string, numRuns: number) {
    const config = _configs.get(configId);
    if (!config) return;

    // Get existing results to preserve values
    const existingResults = _results.get(configId);
    const existingMap = new Map<string, number | null>();
    if (existingResults) {
      for (const m of existingResults.measurements) {
        const key = `${m.runIndex}-${m.replicateIndex}-${m.responseId}`;
        existingMap.set(key, m.value);
      }
    }

    const measurements: Measurement[] = [];

    for (let runIdx = 0; runIdx < numRuns; runIdx++) {
      for (let repIdx = 0; repIdx < config.replicates; repIdx++) {
        for (const response of config.responses) {
          const key = `${runIdx}-${repIdx}-${response.id}`;
          measurements.push({
            runIndex: runIdx,
            replicateIndex: repIdx,
            responseId: response.id,
            value: existingMap.get(key) ?? null, // Preserve existing value
          });
        }
      }
    }

    _results.set(configId, {
      configId,
      measurements,
      completedAt: existingResults?.completedAt,
    });

    await saveState();
  },

  getResults(configId: string): ExperimentResults | undefined {
    return _results.get(configId);
  },

  /**
   * Update a single measurement
   */
  async updateMeasurement(
    configId: string,
    runIndex: number,
    replicateIndex: number,
    responseId: string,
    value: number | null
  ) {
    const results = _results.get(configId);
    if (!results) return;

    const idx = results.measurements.findIndex(
      m => m.runIndex === runIndex &&
           m.replicateIndex === replicateIndex &&
           m.responseId === responseId
    );

    if (idx >= 0) {
      results.measurements[idx].value = value;
      _results.set(configId, { ...results });
      await saveState();
    }
  },

  /**
   * Bulk update measurements (e.g., from CSV import)
   */
  async bulkUpdateMeasurements(configId: string, measurements: Measurement[]) {
    const results = _results.get(configId);
    if (!results) return;

    for (const m of measurements) {
      const idx = results.measurements.findIndex(
        existing => existing.runIndex === m.runIndex &&
                   existing.replicateIndex === m.replicateIndex &&
                   existing.responseId === m.responseId
      );
      if (idx >= 0) {
        results.measurements[idx].value = m.value;
      }
    }

    _results.set(configId, { ...results });
    await saveState();
  },

  /**
   * Check if all measurements are complete
   */
  isResultsComplete(configId: string): boolean {
    const results = _results.get(configId);
    if (!results) return false;
    return results.measurements.every(m => m.value !== null);
  },

  /**
   * Get measurement value for a specific cell
   */
  getMeasurement(
    configId: string,
    runIndex: number,
    replicateIndex: number,
    responseId: string
  ): number | null {
    const results = _results.get(configId);
    if (!results) return null;

    const measurement = results.measurements.find(
      m => m.runIndex === runIndex &&
           m.replicateIndex === replicateIndex &&
           m.responseId === responseId
    );

    return measurement?.value ?? null;
  },

  /**
   * Get all measurements for a run (across all replicates)
   */
  getRunMeasurements(configId: string, runIndex: number, responseId: string): (number | null)[] {
    const results = _results.get(configId);
    const config = _configs.get(configId);
    if (!results || !config) return [];

    return Array.from({ length: config.replicates }, (_, repIdx) => {
      const m = results.measurements.find(
        m => m.runIndex === runIndex &&
             m.replicateIndex === repIdx &&
             m.responseId === responseId
      );
      return m?.value ?? null;
    });
  },

  /**
   * Calculate run average
   */
  getRunAverage(configId: string, runIndex: number, responseId: string): number | null {
    const values = this.getRunMeasurements(configId, runIndex, responseId);
    const validValues = values.filter((v): v is number => v !== null);
    if (validValues.length === 0) return null;
    return validValues.reduce((sum, v) => sum + v, 0) / validValues.length;
  },

  // ========================================
  // Analysis
  // ========================================

  getAnalysis(configId: string): DOEAnalysis | undefined {
    return _analyses.get(configId);
  },

  /**
   * Run DOE analysis via Tauri command
   */
  async runAnalysis(
    configId: string,
    arrayData: number[][],
  ): Promise<DOEAnalysis> {
    const config = _configs.get(configId);
    const results = _results.get(configId);

    if (!config || !results) {
      throw new Error('Configuration or results not found');
    }

    if (!this.isResultsComplete(configId)) {
      throw new Error('All measurements must be entered before analysis');
    }

    // Get the primary response (first one)
    const primaryResponse = config.responses[0];
    if (!primaryResponse) {
      throw new Error('No response variable defined');
    }

    _loading = true;
    _error = null;

    try {
      // Build response data matrix (runs × replicates)
      const numRuns = arrayData.length;
      const responseData: number[][] = [];

      for (let runIdx = 0; runIdx < numRuns; runIdx++) {
        const runValues: number[] = [];
        for (let repIdx = 0; repIdx < config.replicates; repIdx++) {
          const value = this.getMeasurement(configId, runIdx, repIdx, primaryResponse.id);
          if (value === null) {
            throw new Error(`Missing measurement for run ${runIdx + 1}, replicate ${repIdx + 1}`);
          }
          runValues.push(value);
        }
        responseData.push(runValues);
      }

      // Use config's analysis settings with defaults for backwards compatibility
      const settings = config.analysisSettings ?? {
        poolingThreshold: 2.0,
        enablePooling: true,
        minUnpooledFactors: 1,
        confidenceLevel: 0.95,
      };

      const request: DOEAnalysisRequest = {
        arrayData,
        responseData,
        factorIds: config.factors.map(f => f.id),
        factorNames: config.factors.map(f => f.name),
        optimizationType: primaryResponse.optimizationType,
        targetValue: primaryResponse.targetValue,
        poolingThreshold: settings.poolingThreshold,
        enablePooling: settings.enablePooling,
        minUnpooledFactors: settings.minUnpooledFactors,
        confidenceLevel: settings.confidenceLevel,
      };

      const analysis = await invoke<DOEAnalysis>('run_doe_analysis', { request });

      // Add configId and timestamp
      const completeAnalysis: DOEAnalysis = {
        ...analysis,
        configId,
        analyzedAt: new Date().toISOString(),
      };

      _analyses.set(configId, completeAnalysis);

      // Mark results as complete
      const updatedResults = _results.get(configId);
      if (updatedResults) {
        updatedResults.completedAt = new Date().toISOString();
        _results.set(configId, updatedResults);
      }

      await saveState();
      return completeAnalysis;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  // ========================================
  // Utility
  // ========================================

  clearError() {
    _error = null;
  },

  async clear() {
    _configs.clear();
    _results.clear();
    _analyses.clear();
    _currentConfigId = null;
    _draftConfig = null;
    await saveState();
  },

  /**
   * Export config for backup
   */
  exportConfig(configId: string): { config: DOEConfig; results: ExperimentResults; analysis?: DOEAnalysis } | null {
    const config = _configs.get(configId);
    const results = _results.get(configId);
    const analysis = _analyses.get(configId);
    if (!config || !results) return null;
    return { config, results, analysis };
  },

  /**
   * Import config from backup
   */
  async importConfig(data: { config: DOEConfig; results: ExperimentResults; analysis?: DOEAnalysis }) {
    _configs.set(data.config.id, data.config);
    _results.set(data.config.id, data.results);
    if (data.analysis) {
      _analyses.set(data.config.id, data.analysis);
    }
    await saveState();
  },
};
