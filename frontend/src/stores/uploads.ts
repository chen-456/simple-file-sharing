import { ref, type Ref } from 'vue';
import { defineStore } from 'pinia';
import type { FileUploader } from '@/utils/file_uploader';

export const useUploadsStore = defineStore('uploads', () => {
  const uploads: Ref<Map<string, FileUploader>> = ref(new Map());

  return { uploads };
});
