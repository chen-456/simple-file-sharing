<script setup lang="ts">
import { computed, ref, type Ref } from 'vue';
import { useRoute } from 'vue-router';
import FileListView from './FileListView.vue';
import DownloadDialog from './DownloadDialog.vue';
import UploadDialog from './UploadDialog.vue';
import { EventBus } from '@/utils/event_bus';

const route = useRoute();

const currentPath = computed(() => (route.query.path as string) || '');
const breadcrumbDisplay = computed(() => {
  const result = [{ 'title': '/', 'to': '/' }];

  const segments = currentPath.value.split('/');
  if (segments.length === 0)
    return result;

  let curPath = '';
  for (const [i, segment] of segments.entries()) {
    if (i !== 0)
      curPath += '/';
    curPath += segment;
    result.push({ 'title': segment, 'to': '/?path=' + encodeURIComponent(curPath) });
  }
  return result;
});

class DownloadTask {
  id: number;
  path: string;

  constructor(id: number, path: string) {
    this.id = id;
    this.path = path;
  }
}

const downloadTasks: Ref<DownloadTask[]> = ref([]), curDownloadId = ref(0);

function startDownload(path: string) {
  // console.log('Downloading', path);
  const id = curDownloadId.value++;
  downloadTasks.value.push(new DownloadTask(id, path));
}

function onDownloadFinish(id: number) {
  const idx = downloadTasks.value.findIndex(item => item.id == id);
  if (idx !== -1) {
    downloadTasks.value.splice(idx, 1);
  } else {
    console.warn('Failed to remove download task', id);
  }
}

const showUploadDialog = ref(false);

function onUploadFile() {
  showUploadDialog.value = true;
}

function onUploadFinish() {
  showUploadDialog.value = false;
}

EventBus.on('new-file', onUploadFile);
</script>

<template>
  <v-breadcrumbs :items="breadcrumbDisplay">
    <template v-slot:divider>
      <v-icon icon="mdi-chevron-right" />
    </template>
  </v-breadcrumbs>

  <file-list-view :path="currentPath" @download="startDownload" />

  <download-dialog v-for="task in downloadTasks" :key="task.id" :path="task.path" @finish="onDownloadFinish(task.id)" />
  <upload-dialog v-if="showUploadDialog" :parentPath="currentPath" @finish="onUploadFinish" />
</template>
