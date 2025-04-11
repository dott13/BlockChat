<template>
    <aside class="h-screen w-64 bg-primary text-white shadow-lg flex flex-col">
        <div class="p-6 text-2xl font-bold">
            {{ title }}
        </div>
        <nav class="flex-1">
            <ul>
                <li v-for="item in items"  :key="item.route">
                    <NuxtLink
                    :to="item.route"
                    class="block px-6 py-3 hover:bg-secondary transition-colors duration-200"
                    >
                        {{ item.label }}
                    </NuxtLink>
                </li>
                <slot />
            </ul>
        </nav>
        <div class="p-6">
            <slot name="bottom" />
        </div>
    </aside>
</template>

<script setup lang="ts">
import { NuxtLink } from '#components';

    interface SidebarItem {
        label: string;
        route: string;
    }

    const props = withDefaults(defineProps<{
        title?: string;
        items?: SidebarItem[];
        }>(), {
            title: 'Sidebar',
            items: () => [] as SidebarItem[],
    })
</script>