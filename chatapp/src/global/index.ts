import { invoke } from '@tauri-apps/api/core'

let BASE_URL = 'http://127.0.0.1:6688/api'
let SSE_URL = 'http://127.0.0.1:6687/events'

try {
  const config = (await invoke('get_config')) as Interface.IConfig
  BASE_URL = config.server.chat
  SSE_URL = config.server.notification
} catch (err) {
  console.error(err)
}

export { BASE_URL, SSE_URL }
