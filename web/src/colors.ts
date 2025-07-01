export const kindToColor = {
  Footway: "black",
  WithTraffic: "grey",
  "Crossing(Signalized)": "green",
  "Crossing(Zebra)": "green",
  "Crossing(Other)": "green",
  Severance: "red",
};

export const crossingColors = {
  Signalized: "yellow",
  Zebra: "white",
  Other: "brown",
};

export const colorScale = [
  "#CDE594",
  "#80C6A3",
  "#1F9EB7",
  "#186290",
  "#080C54",
];
export const limits = [1, 4, 7, 10, 13, 15];

// Thanks to https://ropensci.github.io/slopes/articles/roadnetworkcycling.html
export let gradientColors = [
  "#267300",
  "#FFAA00",
  "#E60000",
  "#A80000",
  "#730000",
  "#000000",
];
export let gradientLimits = [0, 3, 5, 8, 10, 20, 100];
