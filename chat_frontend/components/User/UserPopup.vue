<template>
    <UiPopup>
        <template #header>
            <h2>{{ editable ? 'Edit User' : 'User Details' }}</h2>
        </template>
        <div>
        <label class="block mb-2">
            First Name:
            <input
             type="text"
             v-model="localUser.first_name"
             :readonly="!editable"
             class="w-full border px-2 py-1 rounded"
            />
        </label>
        <label class="block mb-2">
            Last Name:
            <input
             type="text"
             v-model="localUser.last_name"
             :readonly="!editable"
             class="w-full border px-2 py-1 rounded"
            />
        </label>
        <label class="block mb-2">
            Username:
                <input
                 type="text"
                 v-model="localUser.username"
                 :readonly="!editable"
                 class="w-full border px-2 py-1 rounded"
                />
            </label>
        </div>
        <template #footer>
            <div class="flex justify-end gap-2">
                <UiButton variant="secondary" @click="$emit('close')">Cancel</UiButton>
                <UiButton v-if="editable" variant="primary" @click="save">Save</UiButton>
            </div>
        </template>
    </UiPopup>
</template>

<script setup lang="ts">

const props = defineProps({
    user: {
        type: Object,
        required: true,
    },
    editable: {
        type: Boolean,
        default: false,
    }
})

const emit = defineEmits(['close', 'save'])

const localUser = reactive({ ...props.user })

watch(
    () => props.user,
        (newUser) => {
            Object.assign(localUser, newUser)
    }
)

function save() {
    emit('save', toRaw(localUser))
    emit('close')
}
</script>