<template>
    <div class="flex min-h-screen bg-background">
        <UiSidebar
            title="Admin Panel"
            :items="navItems"
        >
            <template #bottom>
                <button class="w-full px-4 py-2 border border-white rounded hover:bg-secondary transition-colors duration-200">
                    Logout
                </button>
            </template>
        </UiSidebar>
        <main class="flex-1 p-6">
            <slot />
        </main>
    </div>
</template>

<script setup lang="ts">

    const navItems = [
        {label: 'Dashboard', route: '/admin'},
        {label: 'Users', route: '/admin/users'},
        {label: 'Settings', route: '/admin/settings'},
    ]
</script>