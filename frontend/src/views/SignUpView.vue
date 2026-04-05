<template>
  <div class="container py-5">
    <div class="row justify-content-center">
      <div class="col-md-5">
        <div class="card shadow-sm">
          <div class="card-body p-4">
            <h4 class="text-center mb-4" style="color: #1A73E8">CertManager</h4>
            <h5 class="text-center mb-4">アカウント作成</h5>
            <form @submit.prevent="handleSubmit">
              <div class="mb-3">
                <label class="form-label">ユーザー名</label>
                <input v-model="form.username" type="text" class="form-control" required />
                <div v-if="errors.username" class="text-danger small mt-1">{{ errors.username }}</div>
              </div>
              <div class="mb-3">
                <label class="form-label">メールアドレス</label>
                <input v-model="form.email" type="email" class="form-control" required />
                <div v-if="errors.email" class="text-danger small mt-1">{{ errors.email }}</div>
              </div>
              <div class="mb-3">
                <label class="form-label">パスワード</label>
                <input v-model="form.password" type="password" class="form-control" required />
                <div v-if="errors.password" class="text-danger small mt-1">{{ errors.password }}</div>
              </div>
              <div v-if="errors.general" class="alert alert-danger">{{ errors.general }}</div>
              <button type="submit" class="btn btn-primary w-100" :disabled="loading">
                {{ loading ? '作成中...' : 'サインアップ' }}
              </button>
            </form>
            <p class="text-center mt-3 mb-0">
              <router-link to="/signin">アカウントをお持ちの方はこちら</router-link>
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
import { useToast } from '../composables/useToast'

const auth = useAuthStore()
const router = useRouter()
const toast = useToast()

const form = reactive({ username: '', email: '', password: '' })
const errors = reactive<Record<string, string>>({})
const loading = ref(false)

function validate(): boolean {
  Object.keys(errors).forEach((k) => delete errors[k])
  if (form.username.length < 3 || form.username.length > 30) {
    errors.username = 'ユーザー名は3〜30文字で入力してください'
  }
  if (!form.email.includes('@')) {
    errors.email = '有効なメールアドレスを入力してください'
  }
  if (form.password.length < 8) {
    errors.password = 'パスワードは8文字以上で入力してください'
  }
  return Object.keys(errors).length === 0
}

async function handleSubmit() {
  if (!validate()) return
  loading.value = true
  try {
    await auth.signup(form)
    toast.show('アカウントが作成されました', 'success')
    router.push('/signin')
  } catch (e: any) {
    errors.general = e.response?.data?.error || 'エラーが発生しました'
  } finally {
    loading.value = false
  }
}
</script>
