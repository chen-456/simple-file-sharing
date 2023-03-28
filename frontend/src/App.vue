<script setup lang="ts">
import { RouterView } from 'vue-router';
</script>

<template>
  <v-app>
    <v-app-bar color="blue" :elevation="5">
      <v-app-bar-title>Simple file sharing</v-app-bar-title>
      <v-btn icon>
        <v-icon icon="mdi-upload" />
        <v-tooltip activator="parent" location="bottom">Uploads</v-tooltip>
      </v-btn>
      <v-btn icon>
        <v-icon icon="mdi-account" />
        <v-tooltip activator="parent" location="bottom">My account</v-tooltip>
      </v-btn>
    </v-app-bar>
    <v-main>
      <v-container>
        <router-view />
      </v-container>
    </v-main>
  </v-app>
</template>
