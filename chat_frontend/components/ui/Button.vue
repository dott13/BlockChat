<template>
  <button
    :class="buttonClasses"
    :disabled="loading"
    v-bind="$attrs"
  >
    <div class="flex items-center justify-center">
      <svg
        v-if="loading"
        class="animate-spin mr-2 h-5 w-5 text-white"
        xmlns="http://www.w3.org/2000/svg"
        fill="none"
        viewBox="0 0 24 24"
      >
        <circle
          class="opacity-25"
          cx="12"
          cy="12"
          r="10"
          stroke="currentColor"
          stroke-width="4"
        ></circle>
        <path
          class="opacity-75"
          fill="currentColor"
          d="M4 12a8 8 0 018-8v8H4z"
        ></path>
      </svg>
      <span><slot /></span>
    </div>
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
  // Allow different button styles (variants)
  variant: {
    type: String,
    default: 'primary',
  },
})

// Base classes common to all buttons
const baseClasses = 'px-4 py-2 font-semibold rounded transition-colors duration-200 ease-in-out border text-white'

// Variant definitions – extend these as needed for your UI kit
const variants: Record<string, string> = {
  primary: 'bg-blue-500 border-blue-500 hover:bg-blue-600 active:bg-blue-700',
  secondary: 'bg-green-500 border-green-500 hover:bg-green-600 active:bg-green-700',
  danger: 'bg-red-500 border-red-500 hover:bg-red-600 active:bg-red-700',
  // default fallback variant
}

// Disabled state classes
const disabledClasses = 'cursor-not-allowed opacity-75'

// Computed property that builds the final class string
const buttonClasses = computed(() => {
  const variantClasses = variants[props.variant] || variants.primary
  return `${baseClasses} ${variantClasses} ${props.loading ? disabledClasses : ''}`
})
</script>
