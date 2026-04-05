<template>
  <nav class="navbar navbar-expand-lg navbar-dark" style="background-color: #1A73E8">
    <div class="container">
      <router-link class="navbar-brand fw-bold" to="/dashboard">資格取るぞー！</router-link>
      <div v-if="auth.isAuthenticated" class="d-flex align-items-center ms-auto">
        <span class="text-white me-3">{{ auth.user?.username }}</span>
        <button class="btn btn-outline-light btn-sm" @click="handleSignout">
          サインアウト
        </button>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { useAuthStore } from '../../stores/auth'
import { useRouter } from 'vue-router'

const auth = useAuthStore()
const router = useRouter()

async function handleSignout() {
  await auth.signout()
  router.push('/signin')
}
</script>
