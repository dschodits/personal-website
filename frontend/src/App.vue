<script setup>
import { RouterLink, RouterView, useRouter, useRoute } from 'vue-router'
import TabMenu from 'primevue/tabmenu';
import Divider from 'primevue/divider';
import { ref, onMounted, watch } from "vue";

const router = useRouter();
const route = useRoute();

const active = ref(0);
const items = ref([
  {
    label: 'Home',
    icon: 'pi pi-fw pi-home',
    route: '/'
  },
  {
    label: 'Blog',
    icon: 'pi pi-fw pi-book',
    route: '/blog'
  },
  {
    label: 'Projects',
    icon: 'pi pi-fw pi-github',
    route: '/projects'
  },
  {
    label: 'Contact',
    icon: 'pi pi-fw pi-phone',
    route: '/contact'
  },
]);
onMounted(() => {
  active.value = items.value.findIndex((item) => route.path === router.resolve(item.route).path);
})

watch(
  route,
  () => {
    active.value = items.value.findIndex((item) => route.path === router.resolve(item.route).path);
  },
  { immediate: true }
);
</script>

<template>
  <!--Header-->
    <div class="flex justify-center">
      <TabMenu class="lg:p-4" v-model:activeIndex="active" :model="items">
        <template #item="{ label, item, props }">
          <router-link v-if="item.route" v-slot="routerProps" :to="item.route" custom>
            <a :href="routerProps.href" v-bind="props.action" @click="($event) => routerProps.navigate($event)"
              @keydown.enter.space="($event) => routerProps.navigate($event)">
              <span v-bind="props.icon" />
              <span v-bind="props.label">{{ label }}</span>
            </a>
          </router-link>
        </template>
      </TabMenu>
    </div>

    <!--Body-->
    <RouterView/>

    <!--Footer-->
    <Divider class="pt-6"/>
    <div class="flex justify-center">
      © Copyright {{new Date().getFullYear()}}
    </div>
</template>
