<script setup>
import BlogElement from '../components/BlogElement.vue';
</script>

<template>
    <div class="flex flex-row justify-center">
        <div class="flex flex-col">
            <div class="flex justify-center mb-4">
                <p class="text-2xl font-black">Here are my blogs</p>
            </div>
            <BlogElement title="TITEL" :date="new Date()"
                preview="BLOG INHALT BLOG INHALT BLOG INHALT BLOG INHALT BLOG INHALT BLOG INHALT BLOG INHALT" />
            <BlogElement title="TITEL" :date="new Date()" preview="BLOG INHALT" />
        </div>
    </div>
</template>
  
<style></style>
  