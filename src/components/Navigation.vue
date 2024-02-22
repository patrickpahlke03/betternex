<template>
  <v-app-bar :elevation="2" color="blue" density="compact">
    <v-app-bar-title>
      <v-icon icon="mdi-book-open-page-variant-outline"></v-icon>
      BetterNex
    </v-app-bar-title>

    <template v-slot:append>
    </template>

  </v-app-bar>

  <v-navigation-drawer
      permanent
      class="pt-4"

  >
    <v-list density="compact" nav>
      <v-list-item prepend-icon="mdi-home-city" title="Home" value="home" href="#/" active-color="blue"></v-list-item>
      <v-list-item prepend-icon="mdi-archive" title="Archive" value="archive" href="#/archive" active-color="blue"></v-list-item>
      <v-list-item prepend-icon="mdi-school" title="Grades" value="grades" href="#/grades" active-color="blue"></v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>

<script setup>

</script>