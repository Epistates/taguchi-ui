/**
 * Core type definitions for Taguchi UI
 */

// ========================================
// Orthogonal Array Types
// ========================================

export interface OAData {
  id: string;
  runs: number;
  factors: number;
  levels: number[];
  strength: number;
  data: number[][];
  metadata: OAMetadata;
}

export interface OAMetadata {
  name?: string;
  algorithm: string;
  createdAt: string;
  notes?: string;
}

export interface BuildRequest {
  levels: number | number[];
  factors: number;
  strength: number;
  minRuns?: number;
  algorithm?: string;
}

export interface ConstructionOption {
  name: string;
  runs: number;
  maxFactors: number;
  description: string;
  constraints: string[];
}

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
  suggestions: ConstructionOption[];
}

// ========================================
// Analysis Types
// ========================================

export interface VerificationData {
  isValid: boolean;
  claimedStrength: number;
  actualStrength: number;
  issues: VerificationIssue[];
}

export interface VerificationIssue {
  issueType: string;
  description: string;
  location?: IssueLocation;
}

export interface IssueLocation {
  row?: number;
  col?: number;
  columns?: number[];
}

export interface BalanceData {
  factorBalance: boolean[];
  levelCounts: Record<number, number>[];
  expectedCount: number;
}

export interface CorrelationData {
  matrix: number[][];
  factors: number;
}

// ========================================
// Experiment Types
// ========================================

export interface Experiment {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
  arrays: OAData[];
  notes: string;
  tags: string[];
}

// ========================================
// Import/Export Types
// ========================================

export interface ImportValidation {
  runs: number;
  factors: number;
  levels: number[];
  isMixed: boolean;
  estimatedStrength: number;
  warnings: string[];
}

// ========================================
// Catalogue Types
// ========================================

export interface StandardArrayInfo {
  name: string;
  runs: number;
  factors: number;
  levels: number;
  strength: number;
  description: string;
}

// ========================================
// Navigation Types
// ========================================

export interface NavItem {
  id: string;
  label: string;
  href: string;
  icon?: string;
  badge?: string | number;
  children?: NavItem[];
}

export interface NavSection {
  id: string;
  title: string;
  items: NavItem[];
  collapsible?: boolean;
  defaultOpen?: boolean;
}

// ========================================
// DOE (Design of Experiments) Types
// ========================================

/** Optimization goal for response variable */
export type OptimizationType = 'larger-is-better' | 'smaller-is-better' | 'nominal-is-best';

/** Factor configuration for an experiment */
export interface ExperimentFactor {
  id: string;
  name: string;                      // "Temperature"
  unit?: string;                     // "°C"
  levelValues: (number | string)[];  // [100, 150, 200] or ["Low", "Medium", "High"]
}

/** Response variable definition */
export interface ResponseVariable {
  id: string;
  name: string;                      // "Yield"
  unit?: string;                     // "%"
  optimizationType: OptimizationType;
  targetValue?: number;              // For nominal-is-best only
}

/** Complete experiment configuration */
/** Analysis settings for DOE */
export interface AnalysisSettings {
  poolingThreshold: number;          // F-ratio threshold for pooling (default: 2.0)
  enablePooling: boolean;            // Whether to pool insignificant factors (default: true)
  minUnpooledFactors: number;        // Minimum factors to keep unpooled (default: 1)
  confidenceLevel: number;           // For CI: 0.80, 0.90, 0.95, 0.99 (default: 0.95)
}

export const DEFAULT_ANALYSIS_SETTINGS: AnalysisSettings = {
  poolingThreshold: 2.0,
  enablePooling: true,
  minUnpooledFactors: 1,
  confidenceLevel: 0.95,
};

export interface DOEConfig {
  id: string;
  name: string;
  arrayId: string;                   // Reference to OAData
  factors: ExperimentFactor[];
  responses: ResponseVariable[];
  replicates: number;
  analysisSettings: AnalysisSettings; // Analysis configuration
  createdAt: string;
  updatedAt: string;
}

/** Single measurement entry */
export interface Measurement {
  runIndex: number;
  replicateIndex: number;
  responseId: string;
  value: number | null;
}

/** All results for an experiment */
export interface ExperimentResults {
  configId: string;
  measurements: Measurement[];
  completedAt?: string;
}

/** Main effect analysis for a single factor */
export interface MainEffect {
  factorId: string;
  factorName: string;
  levelMeans: number[];              // Mean response at each level
  levelEffects: number[];            // Effect = mean - grand mean
  range: number;                     // Max - Min (factor importance)
  rank: number;                      // 1 = most important
}

/** Signal-to-Noise ratio analysis for a single factor */
export interface SNRatioEffect {
  factorId: string;
  factorName: string;
  levelSnRatios: number[];           // S/N ratio at each level (dB) - camelCase from Rust
  optimalLevel: number;              // Level index with best S/N
}

/** ANOVA table entry for a factor */
export interface ANOVAEntry {
  factorId: string;
  factorName: string;
  sumOfSquares: number;
  degreesOfFreedom: number;
  meanSquare: number;
  fRatio?: number;
  pValue?: number;
  contributionPercent: number;
  pooled: boolean;
}

/** ANOVA results summary */
export interface ANOVAResult {
  entries: ANOVAEntry[];
  errorSs: number;                       // camelCase from Rust's snake_case
  errorDf: number;
  errorMs: number;
  totalSs: number;
  totalDf: number;
}

/** Optimal settings prediction */
export interface OptimalSettings {
  factorLevels: Record<string, number>;  // factorId -> level index
  predictedMean: number;
  predictedSnRatio: number;              // Note: camelCase from Rust's snake_case
  confidenceInterval?: {
    lower: number;
    upper: number;
    level: number;                        // e.g., 0.95 for 95% CI
  };
}

/** Complete DOE analysis results */
export interface DOEAnalysis {
  configId: string;
  grandMean: number;
  snGrandMean: number;               // Grand mean of all S/N ratios
  mainEffects: MainEffect[];
  snRatioEffects: SNRatioEffect[];
  anova: ANOVAResult;
  optimalSettings: OptimalSettings;
  analyzedAt: string;
}

/** Request payload for DOE analysis */
export interface DOEAnalysisRequest {
  arrayData: number[][];             // The OA matrix
  responseData: number[][];          // Runs × Replicates
  factorIds: string[];               // Factor IDs in order
  factorNames: string[];             // Factor names in order
  optimizationType: OptimizationType;
  targetValue?: number;
  poolingThreshold?: number;         // F-ratio threshold (default 2.0)
  enablePooling?: boolean;           // Whether to pool factors (default true)
  minUnpooledFactors?: number;       // Min factors to keep unpooled (default 1)
  confidenceLevel?: number;          // For CI (default 0.95)
}
