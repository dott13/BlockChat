// stores/auth.ts
import { defineStore } from "pinia"
import type { RequestStatus } from "~/types/RequestStatus"
import useApi from '~/composables/useApi'

interface RegisterPayload {
  first_name: string
  last_name: string
  username: string
  password: string
}
 
interface LoginPayload {
  username: string
  password: string
}

export const useAuthStore = defineStore('auth', {
  state: () => ({
    token: null as string | null,
    error: null as string | null,
    user: null as string | null,
    status: "idle" as RequestStatus,
    isRegistered: false, // Track registration status explicitly
  }),
  
  actions: {
    async register(payload: RegisterPayload) {
      this.status = 'loading'
      this.error = null
      
      try {
        const { data, error } = await useApi(`/users/register`, {
          method: 'POST',
          body: payload,
        })
 
        if (error.value) {
          // Extract backend error message if available
          const errMsg = error.value.message || 'Registration failed'
          this.status = 'error'
          this.error = errMsg
          console.error(error.value)
          return null
        }
 
        // Registration successful, but we don't have a token yet
        this.status = 'success'
        this.isRegistered = true
        
        // Store username temporarily to allow middleware bypass
        localStorage.setItem('justRegistered', 'true')
        localStorage.setItem('registeredUser', payload.username)
        
        return data.value
      } catch (err: any) {
        this.status = 'error'
        this.error = err.message || 'Registration failed'
        console.error(err)
        return null
      }
    },
    
    async login(payload: LoginPayload) {
      this.status = 'loading'
      this.error = null
      
      try {
        const { data, error } = await useApi<{ token: string }>(`/users/login`, {
          method: 'POST',
          body: payload,
        })
        
        if (error.value) {
          this.status = 'error'
          this.error = 'Invalid username or password'
          console.error(error.value)
          return null
        }
 
        if (!data.value) {
          throw new Error('No response received from the API')
        }
 
        const response = data.value
        this.token = response.token
        this.user = payload.username
        localStorage.setItem('token', response.token)
        localStorage.setItem('user', payload.username)
        
        // Clear registration flags
        localStorage.removeItem('justRegistered')
        localStorage.removeItem('registeredUser')
        
        this.status = 'success'
        return response
      } catch (err: any) {
        this.status = 'error'
        this.error = 'Invalid username or password'
        console.error(err)
        return null
      }
    },
    
    logout() {
      this.token = null
      this.user = null
      this.status = 'idle'
      this.isRegistered = false
      localStorage.removeItem('token')
      localStorage.removeItem('user')
      localStorage.removeItem('justRegistered')
      localStorage.removeItem('registeredUser')
    },
    
    checkAuth() {
      const token = localStorage.getItem('token')
      const user = localStorage.getItem('user')
      const justRegistered = localStorage.getItem('justRegistered')
      
      if (token) {
        this.token = token
        this.user = user
        this.status = 'success'
        return true
      }
      
      if (justRegistered === 'true') {
        this.isRegistered = true
        this.user = localStorage.getItem('registeredUser')
        return 'justRegistered'
      }
      
      return false
    }
  }
})