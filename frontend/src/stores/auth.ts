import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { authApi } from '../api/auth'
import type { User, SignUpForm, SignInForm } from '../types'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const loading = ref(false)

  const isAuthenticated = computed(() => user.value !== null)

  async function signup(form: SignUpForm) {
    await authApi.signup(form)
  }

  async function signin(form: SignInForm) {
    const { data } = await authApi.signin(form)
    user.value = {
      id: data.user.id,
      username: data.user.username,
      created_at: '',
    }
  }

  async function signout() {
    await authApi.signout()
    user.value = null
  }

  async function fetchMe() {
    try {
      loading.value = true
      const { data } = await authApi.fetchMe()
      user.value = data
    } catch {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  return { user, loading, isAuthenticated, signup, signin, signout, fetchMe }
})
