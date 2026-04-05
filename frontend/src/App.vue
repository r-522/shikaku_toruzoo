<template>
  <div class="d-flex flex-column min-vh-100">
    <AppHeader />
    <main class="flex-grow-1">
      <router-view />
    </main>
    <AppFooter />
    <ToastNotification />
  </div>
</template>

<script setup lang="ts">
import AppHeader from './components/layout/AppHeader.vue'
import AppFooter from './components/layout/AppFooter.vue'
import ToastNotification from './components/common/ToastNotification.vue'
</script>
