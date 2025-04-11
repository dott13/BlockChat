import type { RequestStatus } from "~/types/RequestStatus"

export interface User {
    id: number
    firstName: string
    lastName: string
    username: string
    avatar?: string
}

export const useUsersStore = defineStore('users', {
    state: () => ({
        users: [] as User[],
        status: 'idle' as RequestStatus,
        error: null as string | null,
    }),

    actions: {
        async getUsers() {
            this.status = 'loading'
            try {

                const token = localStorage.getItem('token')

                const {data, error} = await useApi<User[]>('/users/all', {
                    method: 'GET',
                    headers: {
                        Authorization: `Bearer ${token}`
                    }
                })

                if (error?.value) {
                    this.status = 'error'
                    this.error = error.value.message || 'Failed to fetch users'
                    console.error('Fetch users error:', error.value)
                    return
                }

                if (data?.value) {
                    // Force the response to be type any so that we can check for the property.
                    const response = data.value as any;
                    if (Array.isArray(response)) {
                      this.users = response;
                    } else if (response && Array.isArray(response.users)) {
                      this.users = response.users;
                    } else {
                      throw new Error('Unexpected API response format');
                    }
                    this.status = 'success'
                  } else {
                    throw new Error('No data received from API')
                  }
            } catch (err: any) {
                this.status = 'error';
                this.error = err.message || 'Failed to fetch users';
                console.error(err);
            }
        },
    },
})