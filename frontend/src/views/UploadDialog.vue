<script setup lang="ts">
import { ref, type Ref } from 'vue';
import { useUploadsStore } from '@/stores/uploads';
import { ensureConnection } from '@/utils/control';
import { EventBus } from '@/utils/event_bus';
import { FileUploader } from '@/utils/file_uploader';

const props = defineProps<{
  parentPath: string,
}>();
const emit = defineEmits<{
  (event: 'finish'): void,
}>();
const uploadsStore = useUploadsStore();

const visible = ref(true), closable = ref(false);
const inProgress = ref(false);
const uuid: Ref<string | undefined> = ref();
const error: Ref<string | undefined> = ref();
const percentage = ref(0);
const files: Ref<File[]> = ref([]);

async function onUpload() {
  // assert(files.length === 1);
  const file = files.value[0];
  const filePath = props.parentPath.length !== 0 ?
    (props.parentPath + '/' + file.name) :
    file.name;

  console.log(filePath, file.size);
  inProgress.value = true;
  closable.value = false;
  const controlSocket = await ensureConnection();
  const resp = await controlSocket.execute({
    'cmd': 'Upload',
    'path': filePath,
    'size': file.size,
  });
  if (resp.err === null) {
    const uploader = new FileUploader(file, resp.uuid);
    uuid.value = resp.uuid;
    uploadsStore.uploads.set(resp.uuid, uploader);
    uploader.onProgress(() => {
      if (uploader.isRunning()) {
        percentage.value = uploader.getPercentage();
      } else if (uploader.getError() !== undefined) {
        error.value = uploader.getError();
      } else {
        // TODO: show "upload done"
        // TODO: delete current task from uploadsStore
        EventBus.emit('files-changed');
        percentage.value = 1;
        onFinish();
      }
    });
    uploader.start();
    closable.value = true;
  } else {
    error.value = resp.err;
    inProgress.value = false;
    closable.value = true;
  }
}

function onFinish() {
  visible.value = false;
  setTimeout(() => emit('finish'), 1000);
}
</script>

<template>
  <v-dialog v-model="visible" width="400px">
    <v-card width="100%" title="Upload file">
      <template #text>
        <!-- File chooser -->
        <v-form v-if="!inProgress">
          <v-file-input v-model="files" multiple label="Select file" />
        </v-form>

        <!-- Progress -->
        <v-progress-linear v-if="inProgress" :model-value="percentage * 100" />
        <template v-if="inProgress && error === undefined">Uploading...</template>
        <template v-if="inProgress && error !== undefined">
          Error: {{ error }}
        </template>
      </template>

      <v-card-actions>
        <v-spacer />
        <!-- Run in background / Cancel -->
        <v-btn color="blue-darken-1" variant="text" :disabled="closable" @click="onFinish">
          {{ inProgress ? 'Run in background' : 'Cancel' }}
        </v-btn>

        <!-- Upload -->
        <v-btn v-if="!inProgress" color="blue-darken-1" variant="text" :disabled="files.length === 0" @click="onUpload">
          Upload
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
