<template>
  <button
    class="px-4 py-2 font-semibold rounded transition-colors duration-200 ease-in-out bg-red border border-accent text-white hover:brightness-90 active:bg-secondary loading ? 'cursor-not-allowed opacity-75"
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

<script setup>
const props = defineProps({
  loading: {
    type: Boolean,
    default: false,
  },
});
</script>
