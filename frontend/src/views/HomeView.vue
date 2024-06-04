<script setup>
import Image from "primevue/image";
import Card from 'primevue/card';
</script>

<template>
  <main class="space-y-[5rem] lg:px-[16rem]">
    <!--Row to easily center this one-->
    <div class="flex flex-row justify-center">
      <div class="">
        <Card>
          <template #content>
            <Image src="/Banner.png" alt="Image"/>

          </template>

        </Card>

      </div>
    </div>

    <div class="font-sans lg:text-2xl space-y-[5rem] italic lg:font-medium lg:tracking-wide">
      <div class="flex flex-row lg:justify-end text-right">
        <Card class="lg:w-2/3">
          <template #title> About me </template>
          <template #content>
            <p class="m-0">
              I am Daymon Schodits, born and resident in Vienna. I studied IT at TGM, a higher technical institution, 
              which offers advanced technical education in fields like engineering, IT, and electronics. 
              From a young age I was astounded by computers and information technology.
            </p>
          </template>

        </Card>
      </div>
      <div class="flex flex-row lg:justify-start text-left">
        <Card class="lg:w-2/3">
          <template #title> Present </template>
          <template #content>
            <p class="m-0">
              Starting in October I will begin my studies at TU Munich, to further expand my IT skills. Currently I am open for work in IT and I am also available for participation in IT projects.
            </p>
          </template>

        </Card>
      </div>
    </div>
  </main>
</template>
