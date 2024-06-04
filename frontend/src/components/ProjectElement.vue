<script setup>
defineProps({
    picture: {
        type: String,
        required: true
    },
    title: {
        type: String,
        required: true
    },
    subtitle: {
        type: String,
        required: true
    },
    content: {
        type: String,
        required: true
    },

}) 
</script>

<template>
    <div class="flex flex-row justify-center">

        <Card style="width: 25em" class="shadow-lg shadow-gray-600">
            <template #header>
                <img alt="user header" :src="picture" />
            </template>
            <template #title> {{ title }} </template>
            <template #subtitle> {{ subtitle }} </template>
            <template #content>
                <p class="m-0">
                    {{ content }}
                </p>
            </template>
            <template #footer>
                <Button icon="pi pi-check" label="Save" />
                <Button icon="pi pi-check" label="Save" style="margin-left: 0.5em" />

                <Button icon="pi pi-times" label="Cancel" severity="secondary" style="margin-left: 0.5em" />
            </template>
        </Card>
    </div>
</template>

<style scoped></style>
