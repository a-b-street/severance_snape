import { writable, type Writable } from "svelte/store";

// TODO Move to svelte-utils

// Create a store to represent some state about a layer, syncing it to a URL
// query parameter. The parameter missing is equivalent to stringify returning
// null.
export function urlState<T>(params: {
  name: string;
  defaultValue: T;
  stringify: (state: T) => string | null;
  parse: (param: string) => T;
}): Writable<T> {
  let initialValue = params.defaultValue;
  let param = new URLSearchParams(window.location.search).get(params.name);
  if (param != null) {
    try {
      initialValue = params.parse(param);
    } catch (err) {
      console.warn(
        `Parsing URL parameter ${params.name}=${param} failed, using default value: ${err}`,
      );
    }
  }

  let store = writable(initialValue);
  // TODO How do we avoid leaking this?
  store.subscribe((state) => {
    let url = new URL(window.location.href);
    let value = params.stringify(state);
    if (value == null) {
      url.searchParams.delete(params.name);
    } else {
      url.searchParams.set(params.name, value);
    }
    window.history.replaceState(null, "", url.toString());
  });
  return store;
}

// Generates a `parse` function that insists the input belongs to the set of values
// TODO More specific return type
export function enumUrl(values: string[]): (param: string) => string {
  return (param) => {
    if (values.includes(param)) {
      return param;
    }
    throw new Error(`${param} isn't in ${values}`);
  };
}
