<script setup>
import { ref, computed } from 'vue'
import Archive from "./pages/Archive.vue";
import Home from "./pages/Home.vue";
import Navigation from "./components/Navigation.vue";
import Grades from "./pages/Grades.vue";

const routes = {
  '/': Home,
  '/archive': Archive,
  '/grades': Grades
}

const currentPath = ref(window.location.hash)

window.addEventListener('hashchange', () => {
  currentPath.value = window.location.hash
})

const currentView = computed(() => {
  return routes[currentPath.value.slice(1) || '/'] || Home
})

</script>

<template>
  <v-app>
    <Navigation></Navigation>

    <v-main>
      <component :is="currentView" />
    </v-main>
  </v-app>
<!--

  <div class="container">
    <h1>Welcome to Trainex!</h1>


    <a href="#/">Home</a>
    <a href="#/archive">Archive</a>

    <component :is="currentView" />
  </div>-->
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
