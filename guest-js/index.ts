import { invoke } from '@tauri-apps/api/core'

export async function requestReview(): Promise<void> {
  return await invoke<void>('plugin:in-app-review|request_review')
}
