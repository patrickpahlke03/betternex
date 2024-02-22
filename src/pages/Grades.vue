<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const grades = ref([]);
const isLoading = ref(false);
const headers = ref( [
  { title: 'Topic', value: 'name' },
  { title: 'Grade', key: 'grade', value: item => `${Math.round(item.grade * 10)/10}` },
  { title: 'Points', value: 'points' },
  { title: 'Failed attempts', value: 'trys' },
  { title: 'Average', key: 'average', value: item => `${Math.round(item.average * 10)/10}` },
  { title: 'Credit points', value: 'credits' },
  { title: 'Announcement date', value: 'date' },
]);
const search = ref("");

const name = ref("");

onMounted(() => {
  isLoading.value = true;
  getGrades().then(() => {
    isLoading.value = false;
  });

})

async function getGrades() {
  grades.value = await invoke("get_grades");
}

</script>

<template>
  <v-container>
    <v-skeleton-loader type="card" v-if="isLoading"></v-skeleton-loader>

    <v-card class="mt-6"
            elevation="5"
            title="Grades">
      <v-card-text>
        <v-text-field
            v-model="search"
            label="Search"
            prepend-inner-icon="mdi-magnify"
            single-line
            variant="outlined"
            hide-details
        ></v-text-field>

        <v-data-table
            :items="grades"
            :headers="headers"
            :search="search"
            items-per-page="-1"
        >

          <template v-slot:item.name="{ value,item }">
            <v-icon icon="mdi-check" color="green" v-if="item.grade <= 4.0"/> {{ value }}
          </template>
          <template v-slot:bottom></template>
        </v-data-table>
      </v-card-text>
    </v-card>

  </v-container>

</template>