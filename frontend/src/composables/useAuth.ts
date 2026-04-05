import { useAuthStore } from '../stores/auth'
import { useRouter } from 'vue-router'

export function useAuth() {
  const store = useAuthStore()
  const router = useRouter()

  async function handleSignout() {
    await store.signout()
    router.push('/signin')
  }

  return {
    user: store.user,
    isAuthenticated: store.isAuthenticated,
    handleSignout,
  }
}
