import { invoke } from '@tauri-apps/api/core'

export async function ping(value: string): Promise<string | null> {
  return await invoke<{ value?: string }>('plugin:in-app-review|ping', {
    payload: {
      value
    }
  }).then((r) => (r.value ? r.value : null))
}

export async function requestReview(): Promise<void> {
  return await invoke<void>('plugin:in-app-review|request_review')
}
