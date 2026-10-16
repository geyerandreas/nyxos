const publicRoutes = new Set(['/login', '/register'])

export default defineNuxtRouteMiddleware(async (to) => {
  if (publicRoutes.has(to.path)) {
    return
  }

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
  } catch {
    return navigateTo('/login')
  }
})
