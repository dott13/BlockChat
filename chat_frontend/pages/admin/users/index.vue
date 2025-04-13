<template>
    <div>
      <form @submit.prevent="applyFilters" class="flex flex-wrap gap-4 mb-6">
        <input 
          type="text"
          v-model="filters.first_name"
          placeholder="First Name"
          class="border px-2 py-1 rounded"
        />
        <input 
          type="text"
          v-model="filters.last_name"
          placeholder="Last Name"
          class="border px-2 py-1 rounded"
        />
        <input 
          type="text"
          v-model="filters.username"
          placeholder="Username"
          class="border px-2 py-1 rounded"
        />
        <input 
          type="text"
          v-model="filters.chat_name"
          placeholder="Chat Name"
          class="border px-2 py-1 rounded"
        />
        <input 
          type="text"
          v-model="filters.author_username"
          placeholder="Author Username"
          class="border px-2 py-1 rounded"
        />
        <UiButton type="submit" variant="primary">
          Filter
        </UiButton>
      </form>
      <h1 class="text-3xl font-bold mb-4">Users</h1>
      <div v-if="usersStore.status === 'loading'">Loading...</div>
      <div v-if="usersStore.status === 'error'" class="text-red-600">
        Error: {{ usersStore.error }}
      </div>
      <!-- Use the generic table component or any UI component for display -->
      <DataTable v-if="usersStore.status === 'success'" :data="usersStore.users" :columns="columns" />
  </div>
</template>

<script setup lang="ts" >
import DataTable from '~/components/ui/Table/DataTable.vue';
import { useUsersStore } from '~/stores/users';

const usersStore = useUsersStore();

const filters = ref({
  first_name: '',
  last_name: '',
  username: '',
  chat_name: '',
  author_username: '',
})

const columns = [
  { title: 'ID', dataIndex: 'id' },
  { 
    title: 'Name', 
    dataIndex: 'first_name',
    render: (value: string, row: any) => `${row.first_name} ${row.last_name}`
  },
  { title: 'Username', dataIndex: 'username' },
  { 
    title: 'Avatar', 
    dataIndex: 'avatar',
    render: (value: string | null) =>
      value 
        ? `<img src="data:image/jpeg;base64,${value}" alt="Avatar" class="w-16 h-16 rounded-full object-cover" />`
        : ''
  },
]

function applyFilters() {
  usersStore.getUsers(filters.value)
}

onMounted(() => {
    usersStore.getUsers();
})
</script>