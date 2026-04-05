export function useDebounce<T extends (...args: unknown[]) => void>(fn: T, delay = 300) {
  let timer: ReturnType<typeof setTimeout> | null = null

  function debounced(...args: Parameters<T>) {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => fn(...args), delay)
  }

  function cancel() {
    if (timer) clearTimeout(timer)
  }

  return { debounced, cancel }
}
