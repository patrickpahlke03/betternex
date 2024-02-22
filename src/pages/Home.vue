<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
</script>

<template>
  <v-container>
    <v-sheet
        class="d-flex align-center justify-center flex-wrap text-center mx-auto px-4"
        elevation="4"
        height="250"
        rounded
        max-width="800"
        width="100%"
    >
      <div>
        <h2 class="text-h4 font-weight-black text-blue">Congratulations!</h2>

        <div class="text-h5 font-weight-medium mb-2">
          You are officially a part of the BetterNex Community!
        </div>

        <v-btn variant="text" color="blue" href="#/archive">Go to Archive</v-btn>
      </div>
    </v-sheet>
  </v-container>
</template>