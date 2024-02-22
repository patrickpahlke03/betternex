<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { groupBy } from "core-js/full/array/group-by.js";

const archive = ref([]);
const active_archive = ref([]);
const parent_archive = ref([]);
const files = ref([]);
const isLoading = ref(false);
const selected = ref([]);
const headers = [
  { title: 'Name', value: 'name' },
  { title: 'Actions', value: 'actions' },
]
const archivePath = ref("");
const syncProcess = ref(0);
const isSyncing = ref(false);

const name = ref("");

onMounted(() => {
  isLoading.value = true;
  invoke("get_base_sync_path").then(value => {
    archivePath.value = value;
  });
  getArchive().then(() => {
    isLoading.value = false;
  });

})

async function getArchive() {
  archive.value = await invoke("get_archive");
  active_archive.value = archive.value;
}

async function downloadFile(file) {
  await invoke("download_file", {file: file});
}

async function downloadSelected() {
  console.log(selected.value);
  await invoke("download_files", {files: selected.value});
  selected.value = [];
}

async function syncArchive() {
  isSyncing.value = true;
  let timer = setInterval(() => {
    invoke("get_sync_process").then(result => {
      console.log(result);
      syncProcess.value = result;
    });
  }, 1000);
  await invoke("sync_archive", {archive: archive.value, path: archivePath.value});
  clearInterval(timer);
  isSyncing.value = false;
}

function goto (item) {
  parent_archive.value = active_archive.value;
  active_archive.value = item;

  files.value = active_archive.value.files.groupBy( item => item.topic );
  selected.value = [];
}

function goBack () {
  active_archive.value = parent_archive.value;
  parent_archive.value = archive.value;

  if (active_archive.value.files) {
    files.value = active_archive.value.files.groupBy( item => item.topic );
  } else {
    files.value = []
  }

  selected.value = [];
}

</script>

<template>
  <v-container>
    <v-skeleton-loader type="card" v-if="isLoading"></v-skeleton-loader>
    <v-row>
      <v-col>
        <v-card prepend-icon="mdi-reload"
                title="Actions"
                v-if="!isLoading">
          <v-card-text>
            <v-text-field
                v-model="archivePath"
                density="compact"
                placeholder="Archive path"
                prepend-inner-icon="mdi-folder"
                variant="outlined"

            >
            </v-text-field>
            <v-btn color="blue"
                   density="comfortable"
                   variant="elevated"
                   :disabled="isSyncing"
                   @click="syncArchive">
              Sync archive
            </v-btn>
            <v-progress-linear
                class="mt-3"
                v-model="syncProcess"
                color="blue"
                height="25"
                v-if="isSyncing"
            >
              <template v-slot:default="{ value }">
                <strong>{{ Math.ceil(value) }}%</strong>
              </template>
            </v-progress-linear>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
    <v-row>

      <v-col cols="6"
             v-if="active_archive.children">
        <v-card
            class="mx-auto"
            prepend-icon="mdi-dots-horizontal"
            title="Back"
            @click="goBack"
        >
        </v-card>
      </v-col>

      <v-col cols="6"
             v-if="active_archive.children"
             v-for="item in active_archive.children">
        <v-card
            class="mx-auto"
            :title="item.name"
            prepend-icon="mdi-folder"
            @click="goto(item)"
        >
        </v-card>
      </v-col>
      <v-col cols="6"
             v-else
             v-for="item in active_archive">
        <v-card
            class="mx-auto"
            :title="item.name"
            prepend-icon="mdi-folder"
            @click="goto(item)"
        >
        </v-card>
      </v-col>
    </v-row>

    <v-card v-for="topic in files"
            class="mt-6"
            elevation="5"
            :title="topic[0].topic">
      <v-card-text>
        <v-data-table
            v-model="selected"
            :items="topic"
            show-select
            select-strategy="all"
            :headers="headers"
            return-object
            @change="console.log(selected)"
        >
          <template v-slot:footer.prepend>
            <v-toolbar
                density="compact"
                color="transparent"
            >
              <v-btn
                  variant="elevated"
                  color="primary"
                  dark
                  @click="downloadSelected"
              >
                Download selected
              </v-btn>
            </v-toolbar>
          </template>
          <template v-slot:item.name="{ value }">
                <v-icon icon="mdi-file" />{{ value }}
          </template>
          <template v-slot:item.actions="{ item }">
            <v-icon
                size="small"
                @click="downloadFile(item)"
            >
              mdi-download
            </v-icon>
          </template>
        </v-data-table>
      </v-card-text>
    </v-card>

  </v-container>

</template>

<style>
ul {
  text-align: left;
  list-style: none;
}
</style>