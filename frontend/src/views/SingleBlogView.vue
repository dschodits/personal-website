<script setup>
import { ref } from 'vue'
import { useRoute } from 'vue-router'

var options = { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' };

const route = useRoute()
const blog = ref({});
fetchData();
async function fetchData(){
    var response = await fetch('http://localhost:8000/blog/'+route.params.id);
    blog.value = (await response.json());    
}


</script>

<template>
    <div class="flex flex-row justify-center">
        <div class="flex flex-col">
            <div class="flex justify-center mb-4">
                <p class="text-6xl font-black">{{ blog.title }}</p>
                
            </div>
            <p class="text-black italic text-sm pb-5" v-if="blog.date">
                    {{ new Date(blog.date).toLocaleDateString("en-AT", options) }}
            </p>
            <div v-html="blog.content"></div>
            
        </div>
    </div>
</template>
  
<style></style>
  