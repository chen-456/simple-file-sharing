<script setup lang="ts">
import { ref, type Ref } from 'vue';
import { ensureConnection } from '@/utils/control';

const props = defineProps<{
  parentPath: string,
}>();
const emit = defineEmits<{
  (event: 'finish'): void,
}>();

const visible = ref(true), busy = ref(false);
const name = ref('');
const error: Ref<string | undefined> = ref();

async function create() {
  const folderPath = props.parentPath.length !== 0 ?
    (props.parentPath + '/' + name.value) :
    name.value;

  busy.value = true;
  try {
    const controlSocket = await ensureConnection();
    const resp = await controlSocket.execute({
      'cmd': 'CreateDir',
      'path': folderPath,
    });
    if (resp.err === null) {
      finish();
    } else {
      error.value = resp.err;
    }
  } catch (e: any) {
    console.error('Error:', e);
    error.value = e.toString();
  } finally {
    busy.value = false;
  }
}

function finish() {
  visible.value = false;
  setTimeout(() => emit('finish'), 1000);
}
</script>

<template>
  <v-dialog v-model="visible" width="400px">
    <v-card width="100%" title="New folder">
      <template #text>
        <v-text-field v-model="name" label="Folder name" clearable :readonly="busy" />
        <template v-if="error !== undefined">
          <v-icon icon="mdi-alert-outline" />
          &nbsp;Failed to create directory: {{ error }}
        </template>
      </template>

      <!-- Actions -->
      <v-card-actions>
        <v-spacer />
        <v-btn color="primary" :disabled="busy" @click="finish">Cancel</v-btn>
        <v-btn color="primary" :disabled="busy || name.trim().length === 0" @click="create">Create</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
