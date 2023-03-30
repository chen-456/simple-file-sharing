<script setup lang="ts">
import { ref, type Ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import { ensureConnection } from '@/utils/control';
import { useUserStore } from '@/stores/user';

const router = useRouter();
const userStore = useUserStore();

interface DirEntry {
  name: string;
  directory: boolean;
  size: number | null;
}

const props = defineProps<{
  path: string,
}>();
const emit = defineEmits<{
  (event: 'download', path: string): void,
}>();
const curListing: Ref<DirEntry[] | undefined> = ref();

function iconOf(dirEntry: DirEntry): string {
  return dirEntry.directory ? 'mdi-folder' : 'mdi-file';
}

async function fetchDirEntries() {
  if (userStore.current === undefined) {
    curListing.value = undefined;
    return;
  }

  try {
    const controlSocket = await ensureConnection();
    const result = await controlSocket.execute({
      'cmd': 'ListDir',
      'path': props.path,
    });
    if (result.err !== null) {
      alert('Failed to list directory: ' + result.err);
    } else {
      curListing.value = result.entries;
    }
  } catch (e) {
    alert('Failed to list directory: ' + e);
  }
}

function onNavigate(entry: DirEntry) {
  let newPath = props.path;
  if (newPath.length !== 0)
    newPath += '/';
  newPath += entry.name;

  if (entry.directory) {
    router.push('/?path=' + encodeURIComponent(newPath));
  } else {
    emit('download', newPath);
  }
}

fetchDirEntries();
userStore.$subscribe(() => fetchDirEntries());
watch(() => [props.path], () => fetchDirEntries());
</script>

<template>
  <v-list v-if="curListing !== undefined">
    <v-list-item v-for="(entry, i) in curListing" :key="i" :value="entry" active-color="primary" variant="plain"
      @click="onNavigate(entry)">
      <template v-slot:prepend>
        <v-icon :icon="iconOf(entry)" />
      </template>

      <v-list-item-title>{{ entry.name }}</v-list-item-title>
    </v-list-item>
  </v-list>

  <v-container v-if="curListing === undefined">
    <!-- Loading animation -->
    <template v-if="userStore.current !== undefined">
      <v-progress-circular indeterminate />
      &nbsp;Loading entries...
    </template>

    <!-- Log in prompt -->
    <template v-if="userStore.current === undefined">
      <v-icon icon="mdi-alert-outline" />
      &nbsp;Not logged in
    </template>
  </v-container>
</template>
