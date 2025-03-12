import { defineStore } from "pinia"
import type { RequestStatus } from "~/types/RequestStatus"
// Import your composable
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
  }),
  actions: {
    async register(payload: RegisterPayload) {
      this.status = 'loading'
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
  
        this.status = 'success'
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
        this.status = 'success'
        return response
      } catch (err: any) {
        this.status = 'error'
        this.error = 'Invalid username or password'
        console.error(err)
        return null
      }
    },
  }
})
