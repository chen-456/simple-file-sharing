<script setup lang="ts">
import { ref } from 'vue';
import { RouterView } from 'vue-router';
import LoginDialog from './views/LoginDialog.vue';
import { useUserStore } from './stores/user';
import { EventBus } from './utils/event_bus';

const userStore = useUserStore();
const loginDialog = ref();

function onUploadFile() {
  // Send a global event (to HomeView)
  EventBus.emit('new-file');
}

function onNewFolder() {
  // Send a global event (to HomeView)
  EventBus.emit('new-folder');
}

function showUserInfo() {
  if (userStore.current !== undefined) {
    alert('Logged in as: ' + userStore.displayName);
  } else {
    loginDialog.value.show();
  }
}
</script>

<template>
  <v-app>
    <v-app-bar color="blue" :elevation="5">
      <v-app-bar-title>Simple file sharing</v-app-bar-title>
      <v-btn icon @click="onUploadFile">
        <v-icon icon="mdi-file-plus" />
        <v-tooltip activator="parent" location="bottom">Upload new file</v-tooltip>
      </v-btn>
      <v-btn icon @click="onNewFolder">
        <v-icon icon="mdi-folder-plus" />
        <v-tooltip activator="parent" location="bottom">New folder</v-tooltip>
      </v-btn>
      <v-btn icon>
        <v-icon icon="mdi-upload" />
        <v-tooltip activator="parent" location="bottom">Uploads</v-tooltip>
      </v-btn>
      <v-btn icon @click="showUserInfo">
        <v-icon icon="mdi-account" />
        <v-tooltip activator="parent" location="bottom">My account</v-tooltip>
      </v-btn>
    </v-app-bar>
    <v-main>
      <v-container>
        <router-view />
      </v-container>

      <LoginDialog ref="loginDialog" />
    </v-main>
  </v-app>
</template>
