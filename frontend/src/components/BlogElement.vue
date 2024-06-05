<script setup>
var options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
defineProps({
    title: {
        type: String,
        required: true
    },
    date: {
        type: Number,
        required: true
    },
    preview: {
        type: String,
        required: true
    },
    id: {
        type: String,
        required: true
    }
})
</script>

<template>
    <router-link :to="'/blog/' + id">
    <Card class="mb-4">

            <template #title> {{ title }} </template>
            <template #subtitle>
                <p class="text-black">
                    {{ new Date(date).toLocaleDateString("de-AT", options) }}
                </p>
            </template>
            <template #content>
                {{ preview }}
            </template>
        </Card>
    </router-link>
</template>

<style scoped></style>
