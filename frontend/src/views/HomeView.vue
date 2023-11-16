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
              I am Daymon Schodits, born and resident in Vienna.  
            </p>
          </template>

        </Card>
      </div>
      <div class="flex flex-row lg:justify-start text-left">
        <Card class="lg:w-2/3">
          <template #title> Simple Card </template>
          <template #content>
            <p class="m-0">
              Lorem ipsum dolor sit amet, consectetur adipisicing elit. Inventore sed consequuntur error repudiandae
              numquam deserunt quisquam repellat libero asperiores earum nam nobis, culpa ratione quam perferendis esse,
              cupiditate neque
              quas!
            </p>
          </template>

        </Card>
      </div>
    </div>
  </main>
</template>
