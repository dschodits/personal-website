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
    github: {
        type: String,
        required: false
    },
    website: {
        type: String,
        required: false
    }

}) 
</script>

<template>
    <div class="flex flex-row justify-center">

        <Card class="shadow-lg shadow-gray-600 w-96">
            <template #header>
                <div class="flex flex-row justify-center max-h-56 min-h-56">

                    <img class="min-h-56" :src="picture" />
                </div>
            </template>
            <template #title> {{ title }} </template>
            <template #subtitle> {{ subtitle }} </template>
            <template #content>
                <p class="m-0">
                    {{ content }}
                </p>
            </template>
            <template #footer v-if="github || website">
                <div class="flex justify-around">
                    <a v-if="github" :href="github" target="_blank">
                        <Button icon="pi pi-github" label="Github" severity="secondary" style=""  />
                    </a>
                    <Button v-if="website" icon="pi pi-link" label="Website" severity="secondary" style="" />
                </div>

            </template>
        </Card>
    </div>
</template>

<style scoped></style>
