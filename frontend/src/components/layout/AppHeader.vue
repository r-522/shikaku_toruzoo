<template>
  <nav class="navbar navbar-expand-lg navbar-dark" style="background-color: #1A73E8">
    <div class="container">
      <router-link class="navbar-brand fw-bold" to="/dashboard">CertManager</router-link>
      <button
        class="navbar-toggler"
        type="button"
        data-bs-toggle="collapse"
        data-bs-target="#navbarNav"
      >
        <span class="navbar-toggler-icon"></span>
      </button>
      <div class="collapse navbar-collapse" id="navbarNav">
        <ul v-if="auth.isAuthenticated" class="navbar-nav me-auto">
          <li class="nav-item">
            <router-link class="nav-link" to="/dashboard">ダッシュボード</router-link>
          </li>
          <li class="nav-item">
            <router-link class="nav-link" to="/certifications">所持資格</router-link>
          </li>
          <li class="nav-item">
            <router-link class="nav-link" to="/goals">目標</router-link>
          </li>
          <li class="nav-item">
            <router-link class="nav-link" to="/community">コミュニティ</router-link>
          </li>
        </ul>
        <div v-if="auth.isAuthenticated" class="d-flex align-items-center">
          <span class="text-white me-3">{{ auth.user?.username }}</span>
          <button class="btn btn-outline-light btn-sm" @click="handleSignout">
            サインアウト
          </button>
        </div>
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
