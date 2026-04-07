import { ref, onUnmounted } from 'vue'

export function useWebSocket() {
  const ws = ref<WebSocket | null>(null)
  const isConnected = ref(false)

  function connect(token: string, onMessage: (data: unknown) => void) {
    const url = `${window.location.protocol === 'https:' ? 'wss' : 'ws'}://${window.location.host}/ws?token=${token}`
    ws.value = new WebSocket(url)

    ws.value.onopen = () => {
      isConnected.value = true
    }
    ws.value.onclose = () => {
      isConnected.value = false
      // Reconnect after 3 seconds
      setTimeout(() => connect(token, onMessage), 3000)
    }
    ws.value.onmessage = (event) => {
      try {
        const data = JSON.parse(event.data)
        onMessage(data)
      } catch {}
    }
  }

  function disconnect() {
    ws.value?.close()
    ws.value = null
  }

  onUnmounted(disconnect)

  return { isConnected, connect, disconnect }
}
