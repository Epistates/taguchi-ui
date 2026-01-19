<script lang="ts">
  import { onMount } from 'svelte';
  import type { MainEffect, ExperimentFactor } from '$lib/types';
  import * as echarts from 'echarts';

  interface Props {
    effects: MainEffect[];
    factors: ExperimentFactor[];
    grandMean: number;
  }

  let { effects, factors, grandMean }: Props = $props();
  let chartContainer: HTMLDivElement;
  let chart: echarts.ECharts | null = null;

  // Build chart data
  const chartData = $derived(() => {
    // Get max levels to determine x-axis points
    const maxLevels = Math.max(...effects.map(e => e.levelMeans.length));

    const series = effects.map((effect, idx) => {
      const factor = factors.find(f => f.id === effect.factorId);
      const xLabels = factor?.levelValues.map(String) ??
                      effect.levelMeans.map((_, i) => `Level ${i + 1}`);

      return {
        name: effect.factorName,
        type: 'line' as const,
        data: effect.levelMeans.map((mean, i) => [xLabels[i], mean]),
        symbol: 'circle',
        symbolSize: 8,
        lineStyle: { width: 2 },
        emphasis: { focus: 'series' as const },
      };
    });

    return series;
  });

  function updateChart() {
    if (!chart) return;

    const option: echarts.EChartsOption = {
      tooltip: {
        trigger: 'axis',
        formatter: (params: any) => {
          if (!Array.isArray(params)) return '';
          let html = `<strong>${params[0].axisValueLabel}</strong><br/>`;
          for (const p of params) {
            const value = p.value[1];
            if (value == null || !isFinite(value)) {
              html += `${p.marker} ${p.seriesName}: -<br/>`;
              continue;
            }
            const effect = value - grandMean;
            const sign = effect >= 0 ? '+' : '';
            html += `${p.marker} ${p.seriesName}: ${value.toFixed(2)} (${sign}${effect.toFixed(2)})<br/>`;
          }
          html += `<br/><em>Grand Mean: ${grandMean != null && isFinite(grandMean) ? grandMean.toFixed(2) : '-'}</em>`;
          return html;
        },
      },
      legend: {
        data: effects.map(e => e.factorName),
        bottom: 0,
      },
      grid: {
        left: '3%',
        right: '4%',
        bottom: '15%',
        containLabel: true,
      },
      xAxis: {
        type: 'category',
        name: 'Level',
        nameLocation: 'middle',
        nameGap: 30,
        boundaryGap: true,
      },
      yAxis: {
        type: 'value',
        name: 'Mean Response',
        nameLocation: 'middle',
        nameGap: 50,
      },
      series: chartData(),
    };

    chart.setOption(option);
  }

  onMount(() => {
    chart = echarts.init(chartContainer);
    updateChart();

    const resizeObserver = new ResizeObserver(() => {
      chart?.resize();
    });
    resizeObserver.observe(chartContainer);

    return () => {
      resizeObserver.disconnect();
      chart?.dispose();
    };
  });

  $effect(() => {
    if (chart && effects) {
      updateChart();
    }
  });
</script>

<div class="chart-container" bind:this={chartContainer}></div>

<style>
  .chart-container {
    width: 100%;
    height: 300px;
    min-height: 250px;
  }
</style>
