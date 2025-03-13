import { jwtDecode } from 'jwt-decode';
import { defineNuxtRouteMiddleware, navigateTo } from '#imports'

interface JwtPayload {
  exp: number // Expiration time in seconds
  // Add other properties as needed
}

export default defineNuxtRouteMiddleware((to) => {
  if (process.client) {
    const publicPaths = ['/login', '/register']
    let token = localStorage.getItem('token')
    
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
    
    // If no valid token exists and the path isn't public, redirect to login.
    if (!token && !publicPaths.includes(to.path)) {
      return navigateTo('/login')
    }
    
    // Otherwise, allow navigation.
    return
  }
})