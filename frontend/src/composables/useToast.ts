import { ref } from 'vue'

const message = ref('')
const type = ref<'success' | 'error' | 'info'>('info')
const visible = ref(false)
let timer: ReturnType<typeof setTimeout> | null = null

export function useToast() {
  function show(msg: string, t: 'success' | 'error' | 'info' = 'info', duration = 3000) {
    message.value = msg
    type.value = t
    visible.value = true
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => {
      visible.value = false
    }, duration)
  }

  function hide() {
    visible.value = false
  }

  return { message, type, visible, show, hide }
}
