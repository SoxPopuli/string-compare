import { Chart, registerables } from "chart.js";
import { NamedDistance } from "../string-compare/pkg/string_compare";

Chart.register(...registerables);

function make_datasets(
  normalized: NamedDistance[],
  unnormalized: NamedDistance[],
) {
  return [
    {
      label: "Normalized",
      data: normalized,
      borderWidth: 1,
      grouped: false,
    },
    {
      label: "Unnormalized",
      data: unnormalized,
      borderWidth: 1,
      grouped: false,
    },
  ];
}

export function display_chart(
  element: HTMLCanvasElement,
  normalized: NamedDistance[],
  unnormalized: NamedDistance[],
) {
  return new Chart(element, {
    type: "bar",
    data: {
      datasets: make_datasets(normalized, unnormalized),
    },
    options: {
      skipNull: true,
      parsing: {
        xAxisKey: "distance",
        yAxisKey: "name",
      },
      indexAxis: "y",
      responsive: true,
      maintainAspectRatio: false,
      scales: { y: { beginAtZero: true } },
    },
  });
}

export function update_chart(chart: Chart, normalized: NamedDistance[], unnormalized: NamedDistance[]) {
  //@ts-ignore
  chart.data.datasets = make_datasets(normalized, unnormalized)
  chart.update()
}
