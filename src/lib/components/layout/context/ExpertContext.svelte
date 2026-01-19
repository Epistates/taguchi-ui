<script lang="ts">
  interface AlgorithmInfo {
    name: string;
    formula: string;
    description: string;
    bestFor: string;
  }

  interface Props {
    algorithm?: string | null;
  }

  let { algorithm = null }: Props = $props();

  const algorithms: Record<string, AlgorithmInfo> = {
    bose: {
      name: 'Bose Construction',
      formula: 'OA(s², s+1, s, 2)',
      description: 'Uses finite field arithmetic over GF(s) to construct orthogonal arrays. Named after R.C. Bose who developed it in 1947.',
      bestFor: 'Standard strength-2 arrays with prime power levels',
    },
    bush: {
      name: 'Bush Construction',
      formula: 'OA(sᵗ, t+1, s, t)',
      description: 'Generalizes Bose to higher strengths. Produces arrays with exactly t+1 factors where all t-tuples appear equally.',
      bestFor: 'Higher strength arrays (t > 2) requiring stronger balance',
    },
    hadamard: {
      name: 'Hadamard Construction',
      formula: 'OA(4n, 4n-1, 2, 2)',
      description: 'Based on Hadamard matrices where H·Hᵀ = nI. Very efficient for 2-level screening designs.',
      bestFor: '2-level arrays with many factors',
    },
    addelman_kempthorne: {
      name: 'Addelman-Kempthorne',
      formula: 'OA(2s², mixed, mixed, 2)',
      description: 'Creates mixed-level arrays combining 2-level and s-level factors. Based on work by Addelman & Kempthorne (1961).',
      bestFor: 'Experiments with factors having different numbers of levels',
    },
  };

  // Prime powers up to 31 for reference
  const primePowers = [2, 3, 4, 5, 7, 8, 9, 11, 13, 16, 17, 19, 23, 25, 27, 29, 31];

  let currentAlgo = $derived(algorithm ? algorithms[algorithm] : null);
</script>

<div class="expert-context">
  <!-- Selected Algorithm Details -->
  {#if currentAlgo}
    <section class="context-section">
      <h3 class="section-title">Algorithm</h3>
      <div class="algo-info">
        <div class="algo-name">{currentAlgo.name}</div>
        <code class="algo-formula">{currentAlgo.formula}</code>
        <p class="algo-desc">{currentAlgo.description}</p>
        <div class="algo-best-for">
          <span class="best-for-label">Best for:</span>
          <span class="best-for-text">{currentAlgo.bestFor}</span>
        </div>
      </div>
    </section>
  {/if}

  <!-- Prime Powers Reference -->
  <section class="context-section">
    <h3 class="section-title">Prime Powers</h3>
    <p class="section-desc">Valid values for levels (s) parameter</p>
    <div class="prime-powers">
      {#each primePowers as pp}
        <span class="prime-badge">{pp}</span>
      {/each}
    </div>
    <p class="prime-note">
      Prime powers: p<sup>k</sup> where p is prime
    </p>
  </section>

  <!-- Notation Reference -->
  <section class="context-section">
    <h3 class="section-title">OA Notation</h3>
    <dl class="notation-ref">
      <div class="ref-item">
        <dt>N</dt>
        <dd>Number of runs (rows)</dd>
      </div>
      <div class="ref-item">
        <dt>k</dt>
        <dd>Number of factors (columns)</dd>
      </div>
      <div class="ref-item">
        <dt>s</dt>
        <dd>Number of levels per factor</dd>
      </div>
      <div class="ref-item">
        <dt>t</dt>
        <dd>Strength (balance guarantee)</dd>
      </div>
    </dl>
  </section>

  <!-- Formulas Quick Reference -->
  <section class="context-section">
    <h3 class="section-title">Formulas</h3>
    <div class="formulas">
      <div class="formula-item">
        <span class="formula-label">Bose</span>
        <code>N = s², k ≤ s+1</code>
      </div>
      <div class="formula-item">
        <span class="formula-label">Bush</span>
        <code>N = sᵗ, k = t+1</code>
      </div>
      <div class="formula-item">
        <span class="formula-label">Hadamard</span>
        <code>N = 4n, k = 4n-1</code>
      </div>
      <div class="formula-item">
        <span class="formula-label">A-K</span>
        <code>N = 2s²</code>
      </div>
    </div>
  </section>
</div>

<style>
  .expert-context {
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

  .section-desc {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    margin-top: calc(-1 * var(--space-2));
    margin-bottom: var(--space-3);
  }

  /* Algorithm Info */
  .algo-info {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .algo-name {
    font-weight: var(--font-semibold);
    color: var(--color-accent);
  }

  .algo-formula {
    font-family: var(--font-mono);
    font-size: var(--text-lg);
    color: var(--color-text-primary);
    padding: var(--space-2);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
    text-align: center;
  }

  .algo-desc {
    font-size: var(--text-sm);
    color: var(--color-text-secondary);
    line-height: var(--leading-relaxed);
  }

  .algo-best-for {
    padding-top: var(--space-2);
    border-top: 1px solid var(--color-border-light);
  }

  .best-for-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    display: block;
    margin-bottom: 2px;
  }

  .best-for-text {
    font-size: var(--text-sm);
    color: var(--color-text-primary);
    font-weight: var(--font-medium);
  }

  /* Prime Powers */
  .prime-powers {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin-bottom: var(--space-2);
  }

  .prime-badge {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    min-width: 28px;
    padding: 2px 6px;
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    background-color: var(--color-bg-secondary);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
  }

  .prime-note {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    font-style: italic;
  }

  /* Notation Reference */
  .notation-ref {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .ref-item {
    display: flex;
    align-items: baseline;
    gap: var(--space-2);
  }

  .ref-item dt {
    font-family: var(--font-mono);
    font-size: var(--text-sm);
    font-weight: var(--font-bold);
    color: var(--color-accent);
    min-width: 20px;
  }

  .ref-item dd {
    font-size: var(--text-xs);
    color: var(--color-text-secondary);
  }

  /* Formulas */
  .formulas {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }

  .formula-item {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }

  .formula-label {
    font-size: var(--text-xs);
    color: var(--color-text-muted);
    min-width: 60px;
  }

  .formula-item code {
    font-family: var(--font-mono);
    font-size: var(--text-xs);
    color: var(--color-text-primary);
  }
</style>
