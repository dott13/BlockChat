export default defineNuxtRouteMiddleware((to) => {
    if (process.client) {
      const publicPaths = ['/login', '/register']
      if (!publicPaths.includes(to.path) && !localStorage.getItem('token')) {
        return navigateTo('/login')
      }
    }
  })