<template>
  <div class="flex items-center justify-center min-h-screen bg-gray-50">
    <div class="w-full max-w-md p-8 space-y-6 rounded-lg shadow-md bg-white">
      <h1 class="text-2xl font-bold text-center text-gray-700">Login</h1>
      
      <form @submit.prevent="handleLogin" class="space-y-4">
        <!-- Username Input -->
        <div>
          <UiInput
            label="Username"
            v-model="state.credentials.username"
            placeholder="Enter your username"
            :status="state.usernameStatus"
            :message="state.usernameMessage"
            :icon="UserIcon"
            @blur="validateUsername"
          />
        </div>
        
        <!-- Password Input -->
        <div>
          <UiInput
            label="Password"
            type="password"
            v-model="state.credentials.password"
            placeholder="Enter your password"
            :status="state.passwordStatus"
            :message="state.passwordMessage"
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
          :loading="state.pending"
        >
          <div class="flex items-center justify-center">
            <ArrowPathIcon v-if="state.pending" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" />
            {{ state.pending ? 'Logging in...' : 'Login' }}
          </div>
        </UiButton>
      </form>
      
      <div v-if="state.error" class="p-3 bg-red-50 border border-red-200 rounded-md text-center text-red-600 text-sm">
        <div class="flex items-center justify-center">
          <ExclamationCircleIcon class="h-5 w-5 mr-2" />
          {{ state.error }}
        </div>
      </div>
      
      <div v-if="state.data" class="p-3 bg-green-50 border border-green-200 rounded-md text-center text-green-600 text-sm">
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

<script setup>
import { reactive, watch } from 'vue';
import { UserIcon, LockClosedIcon, ArrowPathIcon, ExclamationCircleIcon, CheckCircleIcon } from '@heroicons/vue/24/outline';

const { login } = useAuth();

// Consolidated reactive state
const state = reactive({
  credentials: {
    username: "",
    password: ""
  },
  data: null,
  error: null,
  pending: false,
  usernameStatus: '',
  usernameMessage: 'Username must be at least 3 characters.',
  passwordStatus: '',
  passwordMessage: 'Password should be at least 6 characters long.'
});

// Validate username on input
const validateUsername = () => {
  if (state.credentials.username.length === 0) {
    state.usernameStatus = 'error';
    state.usernameMessage = 'Username is required';
  } else if (state.credentials.username.length < 3) {
    state.usernameStatus = 'warning';
    state.usernameMessage = 'Username should be at least 3 characters';
  } else {
    state.usernameStatus = '';
    state.usernameMessage = '';
  }
};

// Validate password on input
const validatePassword = () => {
  if (state.credentials.password.length === 0) {
    state.passwordStatus = 'error';
    state.passwordMessage = 'Password is required';
  } else if (state.credentials.password.length < 6) {
    state.passwordStatus = 'warning';
    state.passwordMessage = 'Password should be at least 6 characters long';
  } else {
    state.passwordStatus = '';
    state.passwordMessage = '';
  }
};

// Watch for changes to reset error states
watch(() => state.credentials.username, () => {
  if (state.error) {
    state.error = null;
  }
});

watch(() => state.credentials.password, () => {
  if (state.error) {
    state.error = null;
  }
});

const handleLogin = async () => {
  // Validate before submitting
  validateUsername();
  validatePassword();
  
  // Check if there are validation errors
  if (state.usernameStatus === 'error' || state.passwordStatus === 'error') {
    return;
  }
  
  state.pending = true;
  state.error = null;
  state.data = null;
  
  try {
    const res = await login(state.credentials);
    
    if (res.error) {
      state.error = res.error.message;
      
      // Determine which field has the error based on the error message
      if (res.error.message.toLowerCase().includes('username')) {
        state.usernameStatus = 'error';
        state.usernameMessage = res.error.message;
      } else if (res.error.message.toLowerCase().includes('password')) {
        state.passwordStatus = 'error';
        state.passwordMessage = res.error.message;
      } else {
        // Generic error
        state.usernameStatus = 'error';
        state.passwordStatus = 'error';
      }
    } else {
      state.data = res.data;
      // Clear any validation statuses
      state.usernameStatus = '';
      state.usernameMessage = '';
      state.passwordStatus = '';
      state.passwordMessage = '';
      
      // Save token if available
      if (res.data && res.data.token) {
        localStorage.setItem("token", res.data.token);
      }
    }
  } catch (error) {
    state.error = error.message || 'An unexpected error occurred';
  } finally {
    state.pending = false;
  }
};
</script>