/**
 * Analysis store - manages verification, balance, and correlation analysis
 */
import { invoke } from '@tauri-apps/api/core';
import type { VerificationData, BalanceData, CorrelationData, OAData } from '$lib/types';

// Store state
let _verification = $state<VerificationData | null>(null);
let _balance = $state<BalanceData | null>(null);
let _correlation = $state<CorrelationData | null>(null);
let _loading = $state(false);
let _error = $state<string | null>(null);

export const analysisStore = {
  // Getters
  get verification() { return _verification; },
  get balance() { return _balance; },
  get correlation() { return _correlation; },
  get loading() { return _loading; },
  get error() { return _error; },

  // Clear all analysis data
  clear() {
    _verification = null;
    _balance = null;
    _correlation = null;
    _error = null;
  },

  clearError() {
    _error = null;
  },

  // Tauri commands
  async verifyArray(oa: OAData, claimedStrength?: number): Promise<VerificationData> {
    _loading = true;
    _error = null;
    try {
      const result = await invoke<VerificationData>('verify_array', {
        data: oa.data,
        claimedStrength: claimedStrength ?? oa.strength,
      });
      _verification = result;
      return result;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async computeStrength(oa: OAData, maxCheck: number = 6): Promise<number> {
    _loading = true;
    _error = null;
    try {
      return await invoke<number>('compute_array_strength', {
        data: oa.data,
        maxCheck,
      });
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async getBalanceReport(oa: OAData): Promise<BalanceData> {
    _loading = true;
    _error = null;
    try {
      const result = await invoke<BalanceData>('get_balance_report', {
        data: oa.data,
      });
      _balance = result;
      return result;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  async getCorrelationMatrix(oa: OAData): Promise<CorrelationData> {
    _loading = true;
    _error = null;
    try {
      const result = await invoke<CorrelationData>('get_correlation_matrix', {
        data: oa.data,
      });
      _correlation = result;
      return result;
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },

  // Run all analyses
  async runFullAnalysis(oa: OAData): Promise<{
    verification: VerificationData;
    balance: BalanceData;
    correlation: CorrelationData;
  }> {
    _loading = true;
    _error = null;
    try {
      const [verification, balance, correlation] = await Promise.all([
        invoke<VerificationData>('verify_array', {
          data: oa.data,
          claimedStrength: oa.strength,
        }),
        invoke<BalanceData>('get_balance_report', {
          data: oa.data,
        }),
        invoke<CorrelationData>('get_correlation_matrix', {
          data: oa.data,
        }),
      ]);

      _verification = verification;
      _balance = balance;
      _correlation = correlation;

      return { verification, balance, correlation };
    } catch (e) {
      _error = String(e);
      throw e;
    } finally {
      _loading = false;
    }
  },
};
