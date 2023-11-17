<script setup>
var options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };
defineProps({
    title: {
        type: String,
        required: true
    },
    date: {
        type: Date,
        required: true
    },
    preview: {
        type: String,
        required: true
    }
})
</script>

<template>
    <Card class="mb-4">
        <template #title> {{ title }} </template>
        <template #subtitle>
            <p class="text-black">
                {{ date.toLocaleDateString("de-AT", options) }}
            </p>
        </template>
        <template #content>
            {{ preview }}
        </template>
    </Card>
</template>

<style scoped></style>
