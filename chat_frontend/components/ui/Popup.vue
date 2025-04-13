<template>
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-opacity-50">
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-xl max-w-lg w-full p-6 text-white">
            <div class="flex justify-between items-center mb-4">
                <slot name="header">
                    <h2 class="text-xl font-bold text-primary">Popup</h2>
                </slot>
                <button @click="$emit('close')" class="text-gray-700 hover:text-gray-900 text-2xl">&times;</button>
            </div>
            <div class="mb-4">
                <slot />
            </div>
            <div>
                <slot name="footer"/>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
    defineEmits(['close']);
</script>   