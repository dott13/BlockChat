<template>
    <div class="min-h-screen flex flex-col items-center justify-center bg-gray-50 p-4">
      <h1 class="text-3xl font-bold mb-4">Welcome, {{ username }}</h1>
      <p class="text-lg">
        Token Expires: <span class="font-semibold">{{ expTime }}</span>
      </p>
      <p v-if="!token" class="text-red-600 mt-4">
        No valid token found.
      </p>
    </div>
  </template>
  
  <script setup lang="ts">
  import { ref, computed, onMounted } from 'vue'
  import { jwtDecode } from 'jwt-decode'
  
  interface JwtPayload {
    sub: string
    exp: number
  }
  
  const token = ref<string | null>(null)
  const decoded = ref<JwtPayload | null>(null)
  
  // Extract the username (sub) or show a default value.
  const username = computed(() => decoded.value?.sub || 'Guest')
  
  // Convert the expiration time (seconds) into a readable date string.
  const expTime = computed(() => {
    if (decoded.value && decoded.value.exp) {
      const date = new Date(decoded.value.exp * 1000)
      return date.toLocaleString()
    }
    return 'N/A'
  })
  
  onMounted(() => {
    // Only run on client-side (onMounted runs client-side by default)
    token.value = localStorage.getItem('token')
    if (token.value) {
      try {
        decoded.value = jwtDecode<JwtPayload>(token.value)
      } catch (e) {
        console.error('Failed to decode token:', e)
        decoded.value = null
      }
    }
  })
  </script>
  