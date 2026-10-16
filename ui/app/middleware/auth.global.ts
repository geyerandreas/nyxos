const publicRoutes = new Set(['/login', '/register'])

export default defineNuxtRouteMiddleware(async (to) => {
  const isAuthenticated = useState<boolean | null>('auth-status', () => null)
  const config = useRuntimeConfig()
  const headers = import.meta.server
    ? useRequestHeaders(['cookie'])
    : undefined

  try {
    await $fetch('/api/v1/auth/status', {
      baseURL: config.public.apiBase,
      credentials: 'include',
      headers
    })

    isAuthenticated.value = true

    if (publicRoutes.has(to.path)) {
      return navigateTo('/dashboard')
    }
  } catch {
    isAuthenticated.value = false

    if (!publicRoutes.has(to.path)) {
      return navigateTo('/login')
    }
  }
})
