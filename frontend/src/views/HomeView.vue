<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import FileListView from './FileListView.vue';

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
</script>

<template>
  <v-breadcrumbs :items="breadcrumbDisplay">
    <template v-slot:divider>
      <v-icon icon="mdi-chevron-right" />
    </template>
  </v-breadcrumbs>

  <file-list-view :path="currentPath" />
</template>
