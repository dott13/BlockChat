<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-50">
    <div class="w-full max-w-md p-8 space-y-6 rounded-lg shadow-md bg-white">
      <h1 class="text-2xl font-bold text-center text-gray-700">Login</h1>
      
      <form @submit.prevent="handleLogin" class="space-y-4">
        <!-- Username Input -->
        <div>
          <UiInput
            label="Username"
            v-model="form.credentials.username"
            placeholder="Enter your username"
            :status="form.usernameStatus"
            :message="form.usernameMessage"
            :icon="UserIcon"
            @blur="validateUsername"
          />
        </div>
        
        <!-- Password Input -->
        <div>
          <UiInput
            label="Password"
            type="password"
            v-model="form.credentials.password"
            placeholder="Enter your password"
            :status="form.passwordStatus"
            :message="form.passwordMessage"
            :icon="LockClosedIcon"
            @blur="validatePassword"
          />
        </div>
        
        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <input id="remember-me" type="checkbox" class="h-4 w-4 rounded border-gray-300 text-blue-500 focus:ring-blue-500">
            <label for="remember-me" class="ml-2 block text-sm text-gray-700">Remember me</label>
          </div>
          
          <div class="text-sm">
            <a href="#" class="font-medium text-blue-500 hover:text-blue-600">Forgot password?</a>
          </div>
        </div>
        
        <UiButton
          type="submit"
          class="w-full px-4 py-2 font-semibold text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2 transition-colors duration-200"
          :loading="pending"
        >
          <div class="flex items-center justify-center">
            <ArrowPathIcon v-if="pending" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" />
            {{ pending ? 'Logging in...' : 'Login' }}
          </div>
        </UiButton>
      </form>
      
      <!-- Error Message -->
      <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded-md text-center text-red-600 text-sm">
        <div class="flex items-center justify-center">
          <ExclamationCircleIcon class="h-5 w-5 mr-2" />
          {{ error }}
        </div>
      </div>
      
      <!-- Success Message -->
      <div v-if="success" class="p-3 bg-green-50 border border-green-200 rounded-md text-center text-green-600 text-sm">
        <div class="flex items-center justify-center">
          <CheckCircleIcon class="h-5 w-5 mr-2" />
          Login successful!
        </div>
      </div>
      
      <div class="text-center text-sm text-gray-500">
        Don't have an account? 
        <a href="/register" class="font-medium text-blue-500 hover:text-blue-600">Sign up</a>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, computed, watch } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { UserIcon, LockClosedIcon, ArrowPathIcon, ExclamationCircleIcon, CheckCircleIcon } from '@heroicons/vue/24/outline'

// Access the auth store
const authStore = useAuthStore()

// Local reactive state for credentials and validation messages
const form = reactive({
  credentials: {
    username: '',
    password: ''
  },
  usernameStatus: '',
  usernameMessage: 'Username must be at least 3 characters.',
  passwordStatus: '',
  passwordMessage: 'Password should be at least 6 characters long.'
})

// Computed properties from the auth store state
const pending = computed(() => authStore.status === 'loading')
const error = computed(() => authStore.error)
const success = computed(() => authStore.status === 'success' && authStore.token)

// Validate username
const validateUsername = () => {
  if (!form.credentials.username) {
    form.usernameStatus = 'error'
    form.usernameMessage = 'Username is required'
  } else if (form.credentials.username.length < 3) {
    form.usernameStatus = 'warning'
    form.usernameMessage = 'Username should be at least 3 characters'
  } else {
    form.usernameStatus = ''
    form.usernameMessage = ''
  }
}

// Validate password
const validatePassword = () => {
  if (!form.credentials.password) {
    form.passwordStatus = 'error'
    form.passwordMessage = 'Password is required'
  } else if (form.credentials.password.length < 6) {
    form.passwordStatus = 'warning'
    form.passwordMessage = 'Password should be at least 6 characters long'
  } else {
    form.passwordStatus = ''
    form.passwordMessage = ''
  }
}

// Watch for input changes to clear store error
watch(() => form.credentials.username, () => {
  if (authStore.error) authStore.error = null
})
watch(() => form.credentials.password, () => {
  if (authStore.error) authStore.error = null
})

// Handle form submission by calling the store's login action
const handleLogin = async () => {
  validateUsername()
  validatePassword()

  // Abort if there are validation errors
  if (form.usernameStatus === 'error' || form.passwordStatus === 'error') {
    return
  }
  
  await authStore.login(form.credentials)
}
</script>
