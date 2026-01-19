<script lang="ts">
  import { onMount } from 'svelte';
  import type { SNRatioEffect, ExperimentFactor } from '$lib/types';
  import * as echarts from 'echarts';

  interface Props {
    effects: SNRatioEffect[];
    factors: ExperimentFactor[];
  }

  let { effects, factors }: Props = $props();
  let chartContainer: HTMLDivElement;
  let chart: echarts.ECharts | null = null;

  const chartData = $derived(() => {
    return effects.map((effect) => {
      const factor = factors.find(f => f.id === effect.factorId);
      const xLabels = factor?.levelValues.map(String) ??
                      effect.levelSnRatios.map((_, i) => `Level ${i + 1}`);

      return {
        name: effect.factorName,
        type: 'line' as const,
        data: effect.levelSnRatios.map((sn, i) => ({
          // Handle null/infinity values from Rust (JSON serializes infinity as null)
          value: [xLabels[i], sn != null && isFinite(sn) ? sn : 0],
          itemStyle: i === effect.optimalLevel ? {
            color: '#22c55e', // Green color for optimal
            borderWidth: 3,
          } : undefined,
          symbolSize: i === effect.optimalLevel ? 12 : 8,
        })),
        symbol: 'circle',
        symbolSize: 8,
        lineStyle: { width: 2 },
        emphasis: { focus: 'series' as const },
        markPoint: {
          data: [{
            type: 'max' as const,
            name: 'Optimal',
            symbol: 'pin',
            symbolSize: 30,
          }],
          label: {
            formatter: () => 'Opt',
            fontSize: 10,
          },
        },
      };
    });
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
            const effect = effects.find(e => e.factorName === p.seriesName);
            const isOptimal = effect && p.dataIndex === effect.optimalLevel;
            const value = p.value[1];
            const displayValue = value != null && isFinite(value) ? value.toFixed(2) : '-';
            html += `${p.marker} ${p.seriesName}: ${displayValue} dB`;
            if (isOptimal) html += ' <strong>(Optimal)</strong>';
            html += '<br/>';
          }
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
        name: 'S/N Ratio (dB)',
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
