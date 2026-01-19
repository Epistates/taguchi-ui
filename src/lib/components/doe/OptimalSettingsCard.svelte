<script lang="ts">
  import type { OptimalSettings, ExperimentFactor, SNRatioEffect } from '$lib/types';

  interface Props {
    settings: OptimalSettings;
    factors: ExperimentFactor[];
    snEffects: SNRatioEffect[];
    responseName: string;
    responseUnit?: string;
  }

  let { settings, factors, snEffects, responseName, responseUnit }: Props = $props();

  function getFactorSetting(factorId: string) {
    const factor = factors.find(f => f.id === factorId);
    const levelIndex = settings.factorLevels[factorId];

    if (!factor || levelIndex === undefined) {
      return { name: 'Unknown', value: '-' };
    }

    const value = factor.levelValues[levelIndex];
    return {
      name: factor.name,
      value: `${value}${factor.unit ? ` ${factor.unit}` : ''}`,
      levelIndex,
    };
  }

  const orderedFactors = $derived(() => {
    return Object.keys(settings.factorLevels).map(getFactorSetting);
  });
</script>

<div class="optimal-card">
  <div class="card-header">
    <h3>Optimal Settings</h3>
    <span class="badge badge-success">Recommended</span>
  </div>

  <div class="settings-grid">
    {#each orderedFactors() as setting}
      <div class="setting-item">
        <span class="setting-label">{setting.name}</span>
        <span class="setting-value">{setting.value}</span>
      </div>
    {/each}
  </div>

  <div class="divider"></div>

  <div class="prediction-section">
    <div class="prediction-main">
      <span class="prediction-label">Predicted {responseName}</span>
      <span class="prediction-value">
        {settings.predictedMean != null && isFinite(settings.predictedMean) ? settings.predictedMean.toFixed(2) : '-'}
        {#if responseUnit}
          <span class="prediction-unit">{responseUnit}</span>
        {/if}
      </span>
    </div>

    {#if settings.confidenceInterval}
      <div class="confidence-interval">
        <span class="ci-label">
          {(settings.confidenceInterval.level * 100).toFixed(0)}% Confidence Interval
        </span>
        <span class="ci-value">
          [{settings.confidenceInterval.lower != null && isFinite(settings.confidenceInterval.lower) ? settings.confidenceInterval.lower.toFixed(2) : '-'}, {settings.confidenceInterval.upper != null && isFinite(settings.confidenceInterval.upper) ? settings.confidenceInterval.upper.toFixed(2) : '-'}]
          {#if responseUnit}
            <span class="ci-unit">{responseUnit}</span>
          {/if}
        </span>
      </div>
    {/if}

    {#if settings.predictedSnRatio != null && isFinite(settings.predictedSnRatio)}
      <div class="sn-ratio">
        <span class="sn-label">Predicted S/N Ratio</span>
        <span class="sn-value">{settings.predictedSnRatio.toFixed(2)} dB</span>
      </div>
    {/if}
  </div>

  <div class="info-note">
    <svg width="14" height="14" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
      <circle cx="8" cy="8" r="6"/>
      <path d="M8 6v4M8 11v0.5"/>
    </svg>
    <span>These settings are predicted to optimize the response based on main effects analysis.</span>
  </div>
</div>

<style>
  .optimal-card {
    background: linear-gradient(135deg, var(--color-accent-subtle) 0%, var(--color-bg-elevated) 100%);
    border: 2px solid var(--color-accent);
    border-radius: var(--radius-lg);
    padding: var(--space-5);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--space-4);
  }

  .card-header h3 {
    font-size: var(--text-lg);
    font-weight: var(--font-bold);
    color: var(--color-text-primary);
  }

  .settings-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
    gap: var(--space-3);
  }

  .setting-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-3);
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-md);
  }

  .setting-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .setting-value {
    font-size: var(--text-base);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
  }

  .divider {
    height: 1px;
    background-color: var(--color-border);
    margin: var(--space-4) 0;
  }

  .prediction-section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }

  .prediction-main {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .prediction-label {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .prediction-value {
    font-size: var(--text-2xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    font-family: var(--font-mono);
  }

  .prediction-unit {
    font-size: var(--text-base);
    font-weight: normal;
    color: var(--color-text-muted);
  }

  .confidence-interval,
  .sn-ratio {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-2) 0;
    border-top: 1px solid var(--color-border-light);
  }

  .ci-label,
  .sn-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .ci-value,
  .sn-value {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-text-primary);
  }

  .ci-unit {
    color: var(--color-text-muted);
  }

  .info-note {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    margin-top: var(--space-4);
    padding: var(--space-3);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .info-note svg {
    flex-shrink: 0;
    margin-top: 1px;
  }
</style>
