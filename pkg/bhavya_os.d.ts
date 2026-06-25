/* tslint:disable */
/* eslint-disable */

export class Terminal {
    free(): void;
    [Symbol.dispose](): void;
    get_buffer(): string;
    get_cwd(): string;
    handle_down(): string;
    handle_tab(): string;
    handle_up(): string;
    constructor();
    process_key(key: string): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_terminal_free: (a: number, b: number) => void;
    readonly terminal_get_buffer: (a: number) => [number, number];
    readonly terminal_get_cwd: (a: number) => [number, number];
    readonly terminal_handle_down: (a: number) => [number, number];
    readonly terminal_handle_tab: (a: number) => [number, number];
    readonly terminal_handle_up: (a: number) => [number, number];
    readonly terminal_new: () => number;
    readonly terminal_process_key: (a: number, b: number, c: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
