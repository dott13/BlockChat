export default function useApi<T>(endpoint: string, options: any = {}) {
    const config = useRuntimeConfig()
    const url = `${config.public.apiBase}${endpoint}`
    
    return useFetch<T>(url, {
        ...options
      })
}
  