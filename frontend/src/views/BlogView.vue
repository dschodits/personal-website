<script setup>
import BlogElement from '../components/BlogElement.vue';
import { ref,watch } from 'vue'

const blogs = ref({});
fetchData();
async function fetchData(){
    var response = await fetch('http://localhost:8000/blogs');
    blogs.value = (await response.json());    
    console.log(blogs);
}


</script>

<template>
    <div class="flex flex-row justify-center">
        <div class="flex flex-col">
            <div class="flex justify-center mb-4">
                <p class="text-2xl font-black">Here are my blogs</p>
            </div>
            <BlogElement 
                v-for="blog in blogs"
                :title="blog.title"
                :date="blog.date"
                :preview="blog.preview"
                :id="blog.id"
                >

            </BlogElement>
        </div>
    </div>
</template>
  
<style></style>
  