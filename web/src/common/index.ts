export { default as Layout } from "./Layout.svelte";
export { default as Legend } from "./Legend.svelte";
export { default as Loading } from "./Loading.svelte";

export function constructMatchExpression<OutputType>(
  getter: any[],
  map: { [name: string]: OutputType },
  fallback: OutputType
): DataDrivenPropertyValueSpecification<OutputType> {
  let x: any[] = ["match", getter];
  for (let [key, value] of Object.entries(map)) {
    x.push(key);
    x.push(value);
  }
  x.push(fallback);
  return x as DataDrivenPropertyValueSpecification<OutputType>;
}
