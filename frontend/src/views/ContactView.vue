<script setup>
import Card from 'primevue/card';
import ContactElement from '../components/ContactElement.vue';

</script>

<template>
  <div class="flex flex-row justify-center">

    <div class="lg:w-1/3">
      <Card>
        <template #title> Contact </template>
          <template #content>
            <ContactElement msg="Daymon Schodits" icon="pi pi-linkedin" link="https://www.linkedin.com/in/daymon-schodits-45145a226/"/>
            <ContactElement msg="Daymon Schodits" icon="pi pi-github" link="https://github.com/dschodits"/>
            <ContactElement msg="Daymon Schodits" icon="pi pi-envelope"/>
          </template>

      </Card>
    </div>
  </div>
  </template>
  
  <style>

  </style>
  