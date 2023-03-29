<script setup lang="ts">
import { ref } from 'vue';
import { RouterView } from 'vue-router';
import LoginDialog from './views/LoginDialog.vue';

const loginDialog = ref();
</script>

<template>
  <v-app>
    <v-app-bar color="blue" :elevation="5">
      <v-app-bar-title>Simple file sharing</v-app-bar-title>
      <v-btn icon>
        <v-icon icon="mdi-upload" />
        <v-tooltip activator="parent" location="bottom">Uploads</v-tooltip>
      </v-btn>
      <v-btn icon @click="loginDialog.show()">
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
