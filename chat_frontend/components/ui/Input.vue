<template>
  <div class="relative">
    <!-- Container with dynamic border color -->
    <div :class="[
          'border-2 rounded-lg p-2 transition-colors duration-200 ease-in-out focus-within:border-blue-300',
          status === 'error' ? 'border-red-500' : status === 'warning' ? 'border-yellow-500' : 'border-black'
        ]">
      <!-- Floating label overlapping the border -->
      <label class="absolute -top-2 left-4 bg-white px-1 text-xs font-semibold text-gray-700">
        {{ label }}
      </label>
      <div class="flex items-center">
        <!-- Left icon with proper tooltip that appears only when hovering over the icon -->
        <div v-if="status" class="mr-2 relative">
          <div class="cursor-help">
            <ExclamationCircleIcon v-if="status === 'error'" class="h-5 w-5 text-red-500 hover:opacity-80" />
            <ExclamationTriangleIcon v-else-if="status === 'warning'" class="h-5 w-5 text-yellow-500 hover:opacity-80" />
          </div>
          
          <!-- Custom tooltip with wider width -->
          <div class="absolute left-0 bottom-full mb-2 hidden group-hover:block peer-hover:block hover:block z-10 peer-focus:block">
            <div class="bg-white text-xs rounded py-2 px-3 max-w-sm w-64 whitespace-normal border-1 border-black">
              {{ message }}
            </div>
          </div>
        </div>
        
        <!-- The input field -->
        <input
          :value="modelValue"
          @input="updateValue"
          class="flex-1 outline-none bg-transparent"
          v-bind="$attrs"
        />
        
        <!-- Right-side icon if provided -->
        <div v-if="icon" class="ml-2">
          <component :is="icon" />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ExclamationCircleIcon, ExclamationTriangleIcon } from '@heroicons/vue/24/outline';

const props = defineProps({
  label: { type: String, required: true },
  modelValue: { type: String, default: '' },
  // Status can be "", "warning", or "error"
  status: { type: String, default: '' },
  // The error/warning message to show in the tooltip on hover
  message: { type: String, default: '' },
  // Optional right-side icon: accepts Object, String, or Function
  icon: { type: [Object, String, Function], default: null }
});

const emit = defineEmits(['update:modelValue']);

function updateValue(event) {
  emit('update:modelValue', event.target.value);
}
</script>

<style scoped>
  .cursor-help:hover + div {
  display: block;
}
</style>