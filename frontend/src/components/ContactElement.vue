<script setup>
defineProps({
    msg: {
        type: String,
        required: true
    },
    icon: {
        type: String,
        required: true
    },
    link: {
        type: String,
        required: false
    }
})
</script>

<template>
    <div class="flex gap-3 mb-6">
        <a :href="link" class="flex flex-row place-items-center gap-3" target="_blank">
            <i :class="icon" style="font-size: 2rem"></i>
            {{msg}}
        </a>
    </div>
</template>

<style scoped>
</style>
