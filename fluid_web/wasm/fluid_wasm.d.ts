/* tslint:disable */
/* eslint-disable */
/**
*/
export class SimulationWasm {
  free(): void;
/**
*/
  constructor();
/**
* @param {number} x
* @param {number} y
* @param {number} amount
*/
  add_density(x: number, y: number, amount: number): void;
/**
* @param {number} x
* @param {number} y
* @param {number} dx
* @param {number} dy
*/
  add_velocity(x: number, y: number, dx: number, dy: number): void;
/**
*/
  step(): void;
/**
* @param {number} scale
*/
  draw_density(scale: number): void;
/**
*/
  draw_velocity(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_simulationwasm_free: (a: number) => void;
  readonly simulationwasm_new: () => number;
  readonly simulationwasm_add_density: (a: number, b: number, c: number, d: number) => void;
  readonly simulationwasm_add_velocity: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly simulationwasm_step: (a: number) => void;
  readonly simulationwasm_draw_density: (a: number, b: number) => void;
  readonly simulationwasm_draw_velocity: (a: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
