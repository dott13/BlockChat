<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-50">
    <div class="w-full max-w-md p-8 space-y-6 rounded-lg shadow-md bg-white">
      <h1 class="text-2xl font-bold text-center text-gray-700">Register</h1>
      
      <form @submit.prevent="handleRegister" class="space-y-4">
        <!-- First Name and Last Name side by side with reduced spacing -->
        <div class="grid grid-cols-2 gap-2">
          <div>
            <UiInput
              label="First Name"
              v-model="form.first_name"
              placeholder="Enter first name"
              :status="state.firstNameStatus"
              :message="state.firstNameMessage"
              @blur="validateFirstName"
            />
          </div>
          <div>
            <UiInput
              label="Last Name"
              v-model="form.last_name"
              placeholder="Enter last name"
              :status="state.lastNameStatus"
              :message="state.lastNameMessage"
              @blur="validateLastName"
            />
          </div>
        </div>
        
        <!-- Username Input -->
        <div>
          <UiInput
            label="Username"
            v-model="form.username"
            placeholder="Enter your username"
            :status="state.usernameStatus"
            :message="state.usernameMessage"
            @blur="validateUsername"
          />
        </div>
        
        <!-- Password Input -->
        <div>
          <UiInput
            label="Password"
            type="password"
            v-model="form.password"
            placeholder="Enter your password"
            :status="state.passwordStatus"
            :message="state.passwordMessage"
            @blur="validatePassword"
          />
        </div>
        
        <UiButton
          type="submit"
          class="w-full px-4 py-2 font-semibold text-white bg-blue-500 rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-offset-2 transition-colors duration-200"
          :loading="pending"
        >
          <div class="flex items-center justify-center">
            {{ pending ? 'Registering...' : 'Register' }}
          </div>
        </UiButton>
      </form>
      
      <!-- Display error message -->
      <div v-if="error" class="p-3 bg-red-50 border border-red-200 rounded-md text-center text-red-600 text-sm">
        {{ error }}
      </div>
      
      <!-- Display success message -->
      <div v-if="successMessage" class="p-3 bg-green-50 border border-green-200 rounded-md text-center text-green-600 text-sm">
        {{ successMessage }}
      </div>
      
      <div class="text-center text-sm text-gray-500">
        Already have an account?
        <a href="/login" class="font-medium text-blue-500 hover:text-blue-600">Login</a>
      </div>
    </div>
  </div>
</template>
  
<script setup lang="ts">
import { reactive, computed, ref } from 'vue'
import { useAuthStore } from '~/stores/auth'
import { useRouter } from 'vue-router'

// Initialize the auth store and router
const authStore = useAuthStore()
const router = useRouter()
const successMessage = ref('')

// Local reactive form state
const form = reactive({
  first_name: '',
  last_name: '',
  username: '',
  password: ''
})

// Local reactive state for input validation messages and statuses
const state = reactive({
  firstNameStatus: '',
  firstNameMessage: 'First name is required',
  lastNameStatus: '',
  lastNameMessage: 'Last name is required',
  usernameStatus: '',
  usernameMessage: 'Username must be at least 3 characters',
  passwordStatus: '',
  passwordMessage: 'Password must be at least 6 characters'
})

// Computed properties from the auth store
const pending = computed(() => authStore.status === 'loading')
const error = computed(() => authStore.error)

// Validation functions
function validateFirstName() {
  if (!form.first_name.trim()) {
    state.firstNameStatus = 'error'
    state.firstNameMessage = 'First name is required'
    return false
  } else {
    state.firstNameStatus = ''
    state.firstNameMessage = ''
    return true
  }
}

function validateLastName() {
  if (!form.last_name.trim()) {
    state.lastNameStatus = 'error'
    state.lastNameMessage = 'Last name is required'
    return false
  } else {
    state.lastNameStatus = ''
    state.lastNameMessage = ''
    return true
  }
}

function validateUsername() {
  if (!form.username.trim()) {
    state.usernameStatus = 'error'
    state.usernameMessage = 'Username is required'
    return false
  } else if (form.username.length < 3) {
    state.usernameStatus = 'warning'
    state.usernameMessage = 'Username must be at least 3 characters'
    return false
  } else {
    state.usernameStatus = ''
    state.usernameMessage = ''
    return true
  }
}

function validatePassword() {
  if (!form.password) {
    state.passwordStatus = 'error'
    state.passwordMessage = 'Password is required'
    return false
  } else if (form.password.length < 6) {
    state.passwordStatus = 'warning'
    state.passwordMessage = 'Password must be at least 6 characters'
    return false
  } else {
    state.passwordStatus = ''
    state.passwordMessage = ''
    return true
  }
}

// Validate entire form
function validateForm() {
  const isFirstNameValid = validateFirstName()
  const isLastNameValid = validateLastName()
  const isUsernameValid = validateUsername()
  const isPasswordValid = validatePassword()
  
  return isFirstNameValid && isLastNameValid && isUsernameValid && isPasswordValid
}

// Handle registration submission
async function handleRegister() {
  // Validate all fields first
  const isFormValid = validateForm()
  
  // If any field is invalid, abort submission
  if (!isFormValid) {
    return
  }
  
  // Clear any previous error
  authStore.error = null

  try {
    // Call the store's register action
    const result = await authStore.register({
      first_name: form.first_name,
      last_name: form.last_name,
      username: form.username,
      password: form.password
    })
    
    if (result) {
      // Show success message
      successMessage.value = "Registration successful! Redirecting to the application..."
      
      // Navigate to home page after a delay
      setTimeout(() => {
        router.push('/')
      }, 1500)
    }
  } catch (err) {
    console.error("Registration error:", err)
  }
}
</script>