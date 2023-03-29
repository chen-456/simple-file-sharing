import { ref, computed, type Ref } from 'vue';
import { defineStore } from 'pinia';

export class UserInfo {
  username: string;
  password: string | undefined;

  constructor(username: string, password: string | undefined) {
    this.username = username;
    this.password = password;
  }
}

export const useUserStore = defineStore('user', () => {
  const current: Ref<UserInfo | undefined> = ref(undefined);
  const displayName = computed(() => current.value !== undefined ? current.value.username : 'Not logged in');
  function login(user: UserInfo) {
    current.value = user;
  }
  function logout() {
    current.value = undefined;
  }

  return { current, displayName, login, logout };
});
