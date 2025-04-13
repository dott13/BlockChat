<template>
    <table class="w-full border-collapse">
        <thead>
            <tr>
                <th
                    v-for="col in columns"
                    :key="col.dataIndex"
                    class="px-4 py-2 border text-left bg-gray-200"
                    :style="col.width ? `width: ${col.width}` : ''"
                >
                    {{ col.title }}
                </th>
            </tr>
        </thead>
        <tbody>
            <tr
                v-for="(row, rowIndex) in data"
                :key="row.id || rowIndex"
                class="hover:bg-gray-100"
            >
                <td
                    v-for="(col, colIndex) in columns"
                    :key="col.dataIndex"
                    class="border px-4 py-2"
                    :class="{'text-center': col.dataIndex === 'actions'}"
                >
                    <template v-if="col.dataIndex === 'actions'">
                        <slot name="actions" :row="row" :index="rowIndex"></slot>
                    </template>
                    <template v-if="col.render">
                        <span v-html="col.render(row[col.dataIndex], row, rowIndex)"></span>
                    </template>
                    <template v-else>
                        {{ row[col.dataIndex] }}
                    </template>
                </td>
            </tr>
        </tbody>
    </table>
</template>

<script setup lang="ts">
    interface ColumnDefinition {
        title: string;
        dataIndex: string;
        render?: (value: any, row: any, index: number) => string;
        width?: string;
    }
    const props = defineProps<{
        data: any[];
        columns: ColumnDefinition[];
    }>()
</script>