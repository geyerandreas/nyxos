const AUTH_STATUS_PATH = '/api/v1/auth/status'
const AUTH_LOGOUT_PATH = '/api/v1/auth/logout'

export function useAuth() {
  const config = useRuntimeConfig()
  const isAuthenticated = useState<boolean | null>('auth-status', () => null)
  const isLoggingOut = ref(false)

  async function refreshAuthStatus() {
    try {
      await $fetch(AUTH_STATUS_PATH, {
        baseURL: config.public.apiBase,
        credentials: 'include'
      })
      isAuthenticated.value = true
    } catch {
      isAuthenticated.value = false
    }
  }

  async function logout() {
    isLoggingOut.value = true
    try {
      await $fetch(AUTH_LOGOUT_PATH, {
        baseURL: config.public.apiBase,
        method: 'POST',
        credentials: 'include'
      })
      isAuthenticated.value = false
      await navigateTo('/login')
    } finally {
      isLoggingOut.value = false
    }
  }

  return {
    isAuthenticated,
    isLoggingOut,
    refreshAuthStatus,
    logout
  }
}
