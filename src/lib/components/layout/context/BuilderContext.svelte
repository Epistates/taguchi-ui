<script lang="ts">
  import type { ConstructionOption } from '$lib/types';

  interface Props {
    construction?: ConstructionOption | null;
    levels?: number;
    factors?: number;
    strength?: number;
  }

  let { construction = null, levels = 2, factors = 3, strength = 2 }: Props = $props();

  // Calculate notation
  let notation = $derived(
    construction
      ? `OA(${construction.runs}, ${factors}, ${levels}, ${strength})`
      : `OA(?, ${factors}, ${levels}, ${strength})`
  );
</script>

<div class="builder-context">
  <!-- Selected Algorithm -->
  {#if construction}
    <section class="context-section">
      <h3 class="section-title">Construction</h3>
      <div class="construction-info">
        <div class="construction-name">{construction.name}</div>
        <p class="construction-desc">{construction.description}</p>
        <dl class="construction-stats">
          <div class="stat-row">
            <dt>Runs</dt>
            <dd>{construction.runs}</dd>
          </div>
          <div class="stat-row">
            <dt>Max Factors</dt>
            <dd>{construction.maxFactors}</dd>
          </div>
        </dl>
        {#if construction.constraints.length > 0}
          <div class="constraints">
            <span class="constraints-label">Requirements:</span>
            <ul class="constraints-list">
              {#each construction.constraints as constraint}
                <li>{constraint}</li>
              {/each}
            </ul>
          </div>
        {/if}
      </div>
    </section>
  {/if}

  <!-- Notation Reference -->
  <section class="context-section">
    <h3 class="section-title">Array Notation</h3>
    <div class="notation-preview">
      <code class="notation">{notation}</code>
    </div>
    <dl class="notation-ref">
      <div class="ref-item">
        <dt>N (runs)</dt>
        <dd>Number of experimental runs</dd>
      </div>
      <div class="ref-item">
        <dt>k (factors)</dt>
        <dd>Number of factors/columns</dd>
      </div>
      <div class="ref-item">
        <dt>s (levels)</dt>
        <dd>Number of levels per factor</dd>
      </div>
      <div class="ref-item">
        <dt>t (strength)</dt>
        <dd>Degree of balance guarantee</dd>
      </div>
    </dl>
  </section>

  <!-- Quick Tips -->
  <section class="context-section">
    <h3 class="section-title">Tips</h3>
    <ul class="tips-list">
      <li>
        <strong>Strength 2</strong> ensures all pairs of factors appear equally often
      </li>
      <li>
        <strong>More factors</strong> = more runs needed
      </li>
      <li>
        <strong>Start small</strong> with L4 or L9 for initial experiments
      </li>
    </ul>
  </section>
</div>

<style>
  .builder-context {
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

  /* Construction Info */
  .construction-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .construction-name {
    font-weight: var(--font-semibold);
    color: var(--color-accent);
  }

  .construction-desc {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
  }

  .construction-stats {
    display: flex;
    gap: var(--space-4);
    margin-top: var(--space-2);
  }

  .stat-row {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .stat-row dt {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .stat-row dd {
    font-size: var(--text-lg);
    font-weight: var(--font-bold);
    font-family: var(--font-mono);
    color: var(--color-text-primary);
  }

  .constraints {
    margin-top: var(--space-2);
    padding-top: var(--space-2);
    border-top: 1px solid var(--color-border-light);
  }

  .constraints-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  .constraints-list {
    margin-top: var(--space-1);
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }

  .constraints-list li {
    margin-bottom: 2px;
  }

  /* Notation */
  .notation-preview {
    text-align: center;
    padding: var(--space-3);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-md);
    margin-bottom: var(--space-3);
  }

  .notation {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-accent);
  }

  .notation-ref {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .ref-item {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .ref-item dt {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
    font-family: var(--font-mono);
  }

  .ref-item dd {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
  }

  /* Tips */
  .tips-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .tips-list li {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    line-height: var(--leading-relaxed);
  }

  .tips-list strong {
    color: var(--color-text-primary);
  }
</style>
