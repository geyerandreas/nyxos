export default defineNuxtRouteMiddleware(() => {
  const accessToken = useCookie<string | null>('nyxos_access_token')

  if (!accessToken.value) {
    return navigateTo('/login')
  }
})
