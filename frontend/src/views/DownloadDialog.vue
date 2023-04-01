<script setup lang="ts">
import { ensureConnection } from '@/utils/control';
import { computed, ref, type Ref } from 'vue';

const props = defineProps<{
  path: string,
}>();
const emit = defineEmits<{
  (event: 'finish'): void,
}>();
const visible = ref(true);
const uuid: Ref<string | undefined> = ref(), error: Ref<string | undefined> = ref();
const downloadLink = computed(() => import.meta.env.VITE_API_BASE_URL + '/api/file/' + uuid.value);

function finish() {
  // Control commands cannot be terminated, so we just ignore its results
  visible.value = false;
  setTimeout(() => emit('finish'), 1000);
}

async function fetchDownloadLink() {
  try {
    const controlSocket = await ensureConnection();
    const result = await controlSocket.execute({
      'cmd': 'Download',
      'path': props.path,
    });
    if (result.err === null) {
      uuid.value = result.uuid;
      window.open(downloadLink.value);
    } else {
      error.value = result.err;
    }
  } catch (e: any) {
    console.error('Failed to fetch download link:', e);
    error.value = e.toString();
  }
}

// console.log('Download', props.path);
fetchDownloadLink();
</script>

<template>
  <v-dialog v-model="visible" width="400px">
    <v-card width="100%" title="Download file">
      <!-- Download link-->
      <v-card-text v-if="uuid !== undefined">
        If the download does not start automatically, click
        <a :href="downloadLink">here</a>.
      </v-card-text>

      <!-- Progress -->
      <v-card-text v-if="uuid === undefined && error === undefined">
        <v-progress-circular indeterminate color="primary" />
        &nbsp;Fetching download link...
      </v-card-text>

      <!-- Error -->
      <v-card-text v-if="uuid === undefined && error !== undefined">
        <v-progress-circular indeterminate color="primary" />
        &nbsp;Failed to fetch download link: {{ error }}
      </v-card-text>

      <!-- Actions -->
      <v-card-actions>
        <v-spacer />
        <v-btn color="red-darken-2" @click="finish">
          {{ (uuid === undefined && error === undefined) ? 'Cancel' : 'Dismiss' }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
