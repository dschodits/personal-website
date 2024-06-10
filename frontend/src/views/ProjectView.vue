<script setup>
import ProjectElement from '../components/ProjectElement.vue';
</script>

<template>
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 place-content-center">
        <ProjectElement github="https://github.com/dschodits/personal-website" picture="homepage.png" title="Homepage" subtitle="Personal Homepage"
            content="The very website you are looking at right now. This is just a little fun project for myself to get to develop some new skills. I used Rust, Vue3 and TailwindCSS. I personally never used Rust and TailwindCSS before so it was a nice learning experience." />
        <ProjectElement picture="/baeckquem.jpg" title="Baeckquem" subtitle="Bakery Pre-Order System Project"
            content="As our diploma project, our team developed a bakery pre-order system using Laravel, Vue3, and Postgres. The system enables customers to order and pick up baked goods while also providing a web presence for the bakery." />
        <ProjectElement picture="trace_me.png" title="TraceMe" subtitle="A covid tracing system"
            content="A school project from my early days with software development. This project was used for the Open House Day at our school during covid. It tracked which people were in which rooms. This could be traced back and if someone was infected we could notify everyone who was possibly infected." />
    </div>
</template>

<style></style>