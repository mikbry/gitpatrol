import init, { WasmScanner } from 'repo-analyzer-wasm'

let wasmModule: typeof import('repo-analyzer-wasm') | null = null

export async function initWasm() {
  if (!wasmModule) {
    wasmModule = await init()
  }
  return wasmModule
}

export async function scanContent(content: string): Promise<boolean> {
  await initWasm()
  
  const scanner = new WasmScanner()
  scanner.set_content(content)
  return await scanner.scan()
}
