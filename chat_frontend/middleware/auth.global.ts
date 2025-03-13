import { jwtDecode } from 'jwt-decode';
import { defineNuxtRouteMiddleware, navigateTo } from '#imports'

interface JwtPayload {
  exp: number // Expiration time in seconds
  // Add other properties as needed
}

declare global {
  interface Window {
    __registrationTimeout?: ReturnType<typeof setTimeout>;
  }
}

export default defineNuxtRouteMiddleware((to) => {
  if (process.client) {
    const publicPaths = ['/login', '/register']
    let token = localStorage.getItem('token')
    const justRegistered = localStorage.getItem('justRegistered')

    // If token exists, try to decode and check expiration.
    if (token) {
      try {
        const decoded = jwtDecode<JwtPayload>(token)
        if (decoded.exp * 1000 < Date.now()) {
          // Token is expired – remove it.
          localStorage.removeItem('token')
          token = null
          return navigateTo('/login')
        } else {
          // Token is valid.
          // If user navigates to a public path (like /login or /register),
          // redirect them to the home page.
          if (publicPaths.includes(to.path)) {
            return navigateTo('/')
          }
        }
      } catch (e) {
        // If decoding fails, assume token is invalid.
        localStorage.removeItem('token')
        token = null
        return navigateTo('/login')
      }
    }

    // If no valid token exists and the path isn't public and user isn't just registered, redirect to login.
    if (!token && !publicPaths.includes(to.path) && justRegistered !== 'true') {
      return navigateTo('/login')
    }

    // If the user just registered and is trying to access a protected page,
    // allow temporary access and set a timeout to force re-login later.
    if (justRegistered === 'true' && !publicPaths.includes(to.path)) {
      console.log('User just registered, granting temporary access')
      if (!window.__registrationTimeout) {
        window.__registrationTimeout = setTimeout(() => {
          localStorage.removeItem('justRegistered')
          localStorage.removeItem('registeredUser')
          window.location.href = '/login'
        }, 30 * 60 * 1000) // 30 minutes
      }
    }

    // Otherwise, allow navigation.
    return
  }
})
