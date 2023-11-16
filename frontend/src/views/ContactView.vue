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
            <div class="flex justify-center">
              <div>
                <ContactElement msg="Daymon Schodits" icon="pi pi-linkedin" link="https://www.linkedin.com/in/daymon-schodits-45145a226/"/>
                <ContactElement msg="dschodits" icon="pi pi-github" link="https://github.com/dschodits"/>
                <ContactElement msg="dschodits@gmail.com" icon="pi pi-envelope"/>
              </div>
            </div>
            <div class="flex flex-row justify-center">
              <Image src="/wien.jpg" width="400" height="400" alt="Image"/>
            </div>
            <div class="flex justify-center">
              Located in Vienna
            </div>
          </template>

      </Card>
    </div>
  </div>
  </template>
  
  <style>

  </style>
  