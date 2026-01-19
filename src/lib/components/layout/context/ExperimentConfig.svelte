<script lang="ts">
  import { goto } from '$app/navigation';
  import { Button } from '$lib/components/form';
  import type { DOEConfig, OAData } from '$lib/types';

  interface Props {
    config: DOEConfig;
    array?: OAData | null;
  }

  let { config, array = null }: Props = $props();

  function getOptimizationLabel(type: string): string {
    switch (type) {
      case 'larger-is-better': return 'Larger is better';
      case 'smaller-is-better': return 'Smaller is better';
      case 'nominal-is-best': return 'Nominal is best';
      default: return type;
    }
  }

  function formatDate(isoString?: string): string {
    if (!isoString) return 'Unknown';
    return new Date(isoString).toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
    });
  }
</script>

<div class="config-content">
  <!-- Experiment Info -->
  <section class="info-section">
    <h3 class="section-title">Experiment</h3>
    <div class="experiment-name">{config.name}</div>
    {#if array}
      <div class="array-notation">
        OA({array.runs}, {array.factors}, {array.levels[0]}, {array.strength})
      </div>
    {/if}
    <div class="meta-row">
      <span class="meta-label">Created</span>
      <span class="meta-value">{formatDate(config.createdAt)}</span>
    </div>
    <div class="meta-row">
      <span class="meta-label">Replicates</span>
      <span class="meta-value">{config.replicates}</span>
    </div>
  </section>

  <!-- Factors -->
  <section class="info-section">
    <h3 class="section-title">Factors ({config.factors.length})</h3>
    <div class="factor-list">
      {#each config.factors as factor, i}
        <div class="factor-item">
          <span class="factor-name">
            {factor.name}
            {#if factor.unit}
              <span class="factor-unit">({factor.unit})</span>
            {/if}
          </span>
          <span class="factor-levels">
            {factor.levelValues.join(', ')}
          </span>
        </div>
      {/each}
    </div>
  </section>

  <!-- Response Variable -->
  <section class="info-section">
    <h3 class="section-title">Response</h3>
    {#if config.responses[0]}
      {@const response = config.responses[0]}
      <div class="response-info">
        <div class="response-name">
          {response.name}
          {#if response.unit}
            <span class="response-unit">({response.unit})</span>
          {/if}
        </div>
        <div class="response-goal">
          {getOptimizationLabel(response.optimizationType)}
          {#if response.optimizationType === 'nominal-is-best' && response.targetValue !== undefined}
            <span class="target-value">Target: {response.targetValue}</span>
          {/if}
        </div>
      </div>
    {/if}
  </section>

  <!-- Quick Actions -->
  <section class="info-section actions">
    <Button variant="secondary" onclick={() => goto(`/doe/${config.id}/results`)} fullWidth>
      {#snippet children()}
        View Results
      {/snippet}
    </Button>
    {#if array}
      <Button variant="ghost" onclick={() => goto(`/viewer/${array.id}`)} fullWidth>
        {#snippet children()}
          View Array
        {/snippet}
      </Button>
    {/if}
  </section>
</div>

<style>
  .config-content {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .info-section {
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

  .experiment-name {
    font-size: var(--text-lg);
    font-weight: var(--font-semibold);
    color: var(--color-text-primary);
    margin-bottom: var(--space-1);
  }

  .array-notation {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    color: var(--color-accent);
    margin-bottom: var(--space-3);
  }

  .meta-row {
    display: flex;
    justify-content: space-between;
    padding: var(--space-1) 0;
    font-size: var(--text-sm);
  }

  .meta-label {
    color: var(--color-text-secondary);
  }

  .meta-value {
    color: var(--color-text-primary);
    font-weight: var(--font-medium);
  }

  .factor-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .factor-item {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
  }

  .factor-name {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .factor-unit,
  .response-unit {
    font-weight: normal;
    color: var(--color-text-muted);
  }

  .factor-levels {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }

  .response-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .response-name {
    font-size: var(--text-sm);
    font-weight: var(--font-medium);
    color: var(--color-text-primary);
  }

  .response-goal {
    font-size: var(--text-sm);
    color: var(--color-accent);
  }

  .target-value {
    display: block;
    font-family: var(--font-mono);
    color: var(--color-text-secondary);
    margin-top: var(--space-1);
  }

  .actions {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    background-color: transparent;
    padding: 0;
  }
</style>
