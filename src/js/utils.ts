export function createDebounce(callback: Function, delay=300) {
  let timeout: number
  return (...args: any[]) => {
      clearTimeout(timeout)
      //@ts-ignore next-line
      const context = this
      timeout = setTimeout(() => callback.apply(context, args), delay)
  }
}
