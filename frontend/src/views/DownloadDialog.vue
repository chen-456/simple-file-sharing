<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  path: string,
}>();
const emit = defineEmits<{
  (event: 'finish'): void,
}>();
const visible = ref(true);

function finish() {
  visible.value = false;
  setTimeout(() => emit('finish'), 1000);
}

console.log('Download', props.path);
</script>

<template>
  <v-dialog v-model="visible" width="400px">
    <v-card width="100%" title="Download file">
      <template #text>
        <v-progress-circular indeterminate color="primary" />
        &nbsp;Fetching download link...
      </template>
      <template #actions>
        <v-spacer />
        <v-btn color="red-darken-2" @click="finish">Cancel</v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>
