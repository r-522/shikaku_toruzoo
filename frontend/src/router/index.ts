import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/dashboard' },
    { path: '/signup', name: 'SignUp', component: () => import('../views/SignUpView.vue'), meta: { requiresAuth: false } },
    { path: '/signin', name: 'SignIn', component: () => import('../views/SignInView.vue'), meta: { requiresAuth: false } },
    { path: '/dashboard', name: 'Dashboard', component: () => import('../views/DashboardView.vue'), meta: { requiresAuth: true } },
    { path: '/certifications', name: 'Certifications', component: () => import('../views/CertificationView.vue'), meta: { requiresAuth: true } },
    { path: '/goals', name: 'Goals', component: () => import('../views/GoalView.vue'), meta: { requiresAuth: true } },
    { path: '/community', name: 'Community', component: () => import('../views/CommunityView.vue'), meta: { requiresAuth: true } },
    { path: '/community/:id', name: 'UserDetail', component: () => import('../views/UserDetailView.vue'), meta: { requiresAuth: true } },
  ],
})

let initialized = false

router.beforeEach(async (to) => {
  const auth = useAuthStore()

  if (!initialized) {
    await auth.fetchMe()
    initialized = true
  }

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    return '/signin'
  }

  if (!to.meta.requiresAuth && auth.isAuthenticated && (to.name === 'SignIn' || to.name === 'SignUp')) {
    return '/dashboard'
  }
})

export default router
