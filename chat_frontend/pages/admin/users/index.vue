<template>
  <div class="container mx-auto px-4">
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
    <div class="mx-auto">
      <DataTable
        v-if="usersStore.status === 'success'"
        :data="usersStore.users"
        :columns="columns"
      >
        <template #actions="{ row }">
          <div class="flex justify-center gap-2">
            <UiButton variant="secondary" size="small" @click="openPopup(row)">
              <EyeIcon class="h-5 w-5" />
            </UiButton>
            <UiButton variant="danger" size="small" @click="deleteUser(row.id)">
              <TrashIcon class="h-5 w-5" />
            </UiButton>
          </div>
        </template>
      </DataTable>
    </div>
    <UserPopup
      v-if="showPopup"
      :user="selectedUser"
      :editable="isEditable(selectedUser)"
      @close="closePopup"
      @save="saveUser"
    />
</div>
</template>
<script setup lang="ts" >
  import { EyeIcon, TrashIcon } from '@heroicons/vue/24/outline';
  import DataTable from '~/components/ui/Table/DataTable.vue';
import UserPopup from '~/components/User/UserPopup.vue';
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
    {
      title: 'Actions',
      dataIndex: 'actions',
      width: '100px'
    },
  ]
  const showPopup = ref(false);
  const selectedUser = ref<any>(false);
  function openPopup(user: any) {
    selectedUser.value = user
    showPopup.value = true
  }
  function closePopup() {
    showPopup.value = false
    selectedUser.value = null
  }
  function deleteUser(userId: number) {
  // Here, you would call your delete function or API.
    alert(`Deleting user with id: ${userId}`)
  }
  function isEditable(user: any): boolean {
    return true
  }
  function saveUser(updatedUser: any) {
    alert(`userEdited ${updatedUser.id}`)
    closePopup()
  }
  function applyFilters() {
    usersStore.getUsers(filters.value)
  } 
  onMounted(() => {
    usersStore.getUsers();
  })
</script>