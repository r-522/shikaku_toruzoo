<template>
  <div class="container py-5">
    <div class="row justify-content-center">
      <div class="col-md-5">
        <div class="card shadow-sm">
          <div class="card-body p-4">
            <h4 class="text-center mb-4" style="color: #1A73E8">資格取るぞー！</h4>
            <h5 class="text-center mb-4">サインイン</h5>
            <form @submit.prevent="handleSubmit">
              <div class="mb-3">
                <label class="form-label">メールアドレス</label>
                <input v-model="form.email" type="email" class="form-control" required />
              </div>
              <div class="mb-3">
                <label class="form-label">パスワード</label>
                <input v-model="form.password" type="password" class="form-control" required />
              </div>
              <div v-if="error" class="alert alert-danger">{{ error }}</div>
              <button type="submit" class="btn btn-primary w-100" :disabled="loading">
                {{ loading ? 'サインイン中...' : 'サインイン' }}
              </button>
            </form>
            <p class="text-center mt-3 mb-0">
              <router-link to="/signup">アカウントを作成する</router-link>
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const auth = useAuthStore()
const router = useRouter()

const form = reactive({ email: '', password: '' })
const error = ref('')
const loading = ref(false)

async function handleSubmit() {
  error.value = ''
  loading.value = true
  try {
    await auth.signin(form)
    router.push('/dashboard')
  } catch (e: any) {
    error.value = e.response?.data?.error || 'メールアドレスまたはパスワードが正しくありません'
  } finally {
    loading.value = false
  }
}
</script>
