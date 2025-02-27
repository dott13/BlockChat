<template>
    <div class="relative">
        <div class="border-1 border-black rounded-lg p-2 transition-colors duration-200 ease-in-out focus-within:border-blue-300">
            <label class="absolute -top-2 left-4 bg-[#EFFFFB] px-1 text-xs font-semibold text-gray-700">
                {{ label }}
            </label>
            <input
                :value="modelValue"
                @input="updateValue"
                class="w-full mt-1 outline-none bg-transparent"
                v-bind="$attrs"
            />
        </div>
    </div>
</template>

<script setup>
    const props = defineProps({
        label: {
            type: String,
            required: true
        },
        modelValue: {
        type: String,
        default: ''
        }
    });

    const emit = defineEmits(['update:modelValue']);

    function updateValue(event) {
        emit('update:modelValue', event.target.value);
}
</script>
