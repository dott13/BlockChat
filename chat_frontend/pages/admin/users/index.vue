<template>
    <div>
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
onMounted(() => {
    usersStore.getUsers();
})
</script>