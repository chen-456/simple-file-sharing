<script setup lang="ts">
import { ref } from 'vue';
import { useUserStore, UserInfo } from '@/stores/user';
import { ensureConnection } from '@/utils/control';

const store = useUserStore();
const visible = ref(true), busy = ref(false);
const username = ref(''), password = ref('');

function show() {
  visible.value = true;
}

function required(v: string) {
  return !!v || 'Field is required';
}

async function doLoginOrRegister(register: boolean) {
  if (username.value.length === 0 || password.value.length === 0) {
    alert('Missing required fields');
    return;
  }

  busy.value = true;
  try {
    const controlSocket = await ensureConnection();
    await controlSocket.execute({
      'cmd': register ? 'RegisterPwd' : 'LoginPwd',
      'username': username.value,
      'password': password.value,
    });
    // Login successful
    store.login(new UserInfo(username.value, password.value));
    visible.value = false;
    // Clear the form
    username.value = '';
    password.value = '';
  } catch (e) {
    alert('Error occurred: ' + e);
  } finally {
    busy.value = false;
  }
}

defineExpose({ show });
</script>

<template>
  <v-dialog v-model="visible" width="400px">
    <v-card class="mx-auto px-6 py-8" width="100%">
      <v-card-title>
        <span class="text-h5">Log in</span>
      </v-card-title>
      <v-form @submit.prevent="void (0)">
        <v-text-field v-model="username" label="Username" :rules="[required]" clearable :readonly="busy" />
        <v-text-field v-model="password" label="Password" type="password" :rules="[required]" clearable
          :readonly="busy" />
        <br />
        <v-btn type="submit" block size="large" color="success" variant="elevated" :disabled="busy"
          @click="doLoginOrRegister(false)">Login</v-btn>
        <br />
        <v-btn type="submit" block size="large" color="blue" variant="elevated" :disabled="busy"
          @click="doLoginOrRegister(true)">Register</v-btn>
        <br />
        <v-btn color="failed" block size="large" variant="elevated" @click="visible = false">Cancel</v-btn>
      </v-form>
    </v-card>
  </v-dialog>
</template>
