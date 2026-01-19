<script lang="ts">
  import { arrayStore } from '$lib/stores/arrays.svelte';
  import { doeStore } from '$lib/stores/doe.svelte';

  // Stats derived from stores
  let stats = $derived({
    totalArrays: arrayStore.list.length,
    totalExperiments: doeStore.configList.length,
    completedExperiments: doeStore.configList.filter(c => {
      const analysis = doeStore.analyses.get(c.id);
      return analysis !== undefined;
    }).length,
    inProgressExperiments: doeStore.configList.filter(c => {
      const analysis = doeStore.analyses.get(c.id);
      const results = doeStore.results.get(c.id);
      return !analysis && results && results.measurements.some(m => m.value !== null);
    }).length,
  });
</script>

<div class="dashboard-context">
  <!-- Statistics -->
  <section class="context-section">
    <h3 class="section-title">Your Workspace</h3>
    <div class="stats-grid">
      <div class="stat-item">
        <span class="stat-value">{stats.totalArrays}</span>
        <span class="stat-label">Arrays</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{stats.totalExperiments}</span>
        <span class="stat-label">Experiments</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{stats.completedExperiments}</span>
        <span class="stat-label">Analyzed</span>
      </div>
      <div class="stat-item">
        <span class="stat-value">{stats.inProgressExperiments}</span>
        <span class="stat-label">In Progress</span>
      </div>
    </div>
  </section>

  <!-- Getting Started -->
  <section class="context-section">
    <h3 class="section-title">Getting Started</h3>
    <ol class="steps-list">
      <li class="step">
        <span class="step-number">1</span>
        <div class="step-content">
          <span class="step-title">Select an Array</span>
          <span class="step-desc">Browse the catalogue or build a custom array</span>
        </div>
      </li>
      <li class="step">
        <span class="step-number">2</span>
        <div class="step-content">
          <span class="step-title">Configure Experiment</span>
          <span class="step-desc">Define factors, levels, and response variables</span>
        </div>
      </li>
      <li class="step">
        <span class="step-number">3</span>
        <div class="step-content">
          <span class="step-title">Enter Results</span>
          <span class="step-desc">Record measurements from your experiments</span>
        </div>
      </li>
      <li class="step">
        <span class="step-number">4</span>
        <div class="step-content">
          <span class="step-title">Analyze</span>
          <span class="step-desc">Find optimal settings with statistical analysis</span>
        </div>
      </li>
    </ol>
  </section>

  <!-- Quick Reference -->
  <section class="context-section">
    <h3 class="section-title">Quick Reference</h3>
    <dl class="reference-list">
      <div class="reference-item">
        <dt>OA(N, k, s, t)</dt>
        <dd>N runs, k factors, s levels, strength t</dd>
      </div>
      <div class="reference-item">
        <dt>Strength</dt>
        <dd>How many factors are balanced together</dd>
      </div>
      <div class="reference-item">
        <dt>S/N Ratio</dt>
        <dd>Signal-to-noise ratio measures robustness</dd>
      </div>
      <div class="reference-item">
        <dt>Main Effect</dt>
        <dd>Average impact of each factor level</dd>
      </div>
    </dl>
  </section>

  <!-- Keyboard Shortcuts -->
  <section class="context-section">
    <h3 class="section-title">Shortcuts</h3>
    <ul class="shortcut-list">
      <li><kbd>Cmd</kbd><kbd>K</kbd> <span>Command palette</span></li>
      <li><kbd>G</kbd><kbd>C</kbd> <span>Go to Catalogue</span></li>
      <li><kbd>G</kbd><kbd>B</kbd> <span>Go to Builder</span></li>
      <li><kbd>G</kbd><kbd>D</kbd> <span>Go to DOE</span></li>
    </ul>
  </section>
</div>

<style>
  .dashboard-context {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .context-section {
    background-color: var(--color-bg-primary);
    border-radius: var(--radius-md);
    padding: var(--space-3);
  }

  .section-title {
    font-size: var(--text-xs);
    font-weight: var(--font-semibold);
    color: var(--color-text-muted);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin-bottom: var(--space-3);
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-2);
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
  }

  .stat-value {
    font-size: var(--text-xl);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    font-family: var(--font-mono);
  }

  .stat-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  /* Steps */
  .steps-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .step {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
  }

  .step-number {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-accent);
    color: var(--color-text-inverse);
    font-size: var(--text-xs);
    font-weight: var(--font-bold);
    border-radius: var(--radius-full);
    flex-shrink: 0;
  }

  .step-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .step-title {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .step-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  /* Reference */
  .reference-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .reference-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .reference-item dt {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
    font-family: var(--font-mono);
  }

  .reference-item dd {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  /* Shortcuts */
  .shortcut-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .shortcut-list li {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  kbd {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 2px 5px;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    background-color: var(--color-bg-tertiary);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-secondary);
  }
</style>
