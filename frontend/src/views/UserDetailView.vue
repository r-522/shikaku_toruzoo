<!-- ============================================================
views/UserDetailView.vue — ユーザー詳細画面
============================================================
このファイルはコミュニティの特定ユーザーの詳細情報を表示する画面。
所持資格タブと目標タブで切り替えて表示する。

【URL パラメータの取得】
`useRoute()` で現在の URL 情報を取得できる。
`route.params.id` で `/community/:id` の `:id` 部分を取得する。

【raw JSON データのヘルパー関数】
バックエンドの community_service.rs は詳細データを
Rust の serde_json::Value のまま返すため、
フロントエンドでは TypeScript の `Certification`/`Goal` 型と
Supabase の生 JSON（`TBL_MASTER.masnm` 等）の両方を扱う可能性がある。
`getCertName()` 等のヘルパー関数でどちらの形式でも対応する。

【タブ UI】
Bootstrap の `nav nav-tabs` クラスを使ったタブ。
`:class="{ active: tab === 'certs' }"` でアクティブなタブを強調表示。
`tab` の ref 値を切り替えることで `v-if` がどちらのコンテンツを表示するか制御する。
============================================================ -->
<template>
  <div class="container py-4">
    <!-- ローディング中 -->
    <div v-if="communityStore.loading" class="text-center py-5">
      <div class="spinner-border text-primary"></div>
    </div>

    <!-- データ取得後（detail が null でない場合） -->
    <!-- `v-else-if="detail"` は loading が false かつ detail が存在する場合に表示 -->
    <template v-else-if="detail">
      <!-- ヘッダー: 戻るボタン・ユーザー名・グッドマーク -->
      <div class="d-flex align-items-center gap-2 mb-4">
        <!-- `&larr;` = ← （左向き矢印の HTML 文字参照） -->
        <router-link to="/community" class="btn btn-outline-secondary btn-sm">&larr; 戻る</router-link>
        <h3 class="mb-0">{{ detail.username }}</h3>
        <!-- グッドマーク: has_good_mark が true の場合に ✓ バッジを表示 -->
        <GoodMark :has-good-mark="detail.has_good_mark" />
      </div>

      <!-- タブナビゲーション -->
      <!-- `nav nav-tabs`: Bootstrap の水平タブスタイル -->
      <ul class="nav nav-tabs mb-3">
        <li class="nav-item">
          <!-- `:class="{ active: tab === 'certs' }"` でアクティブタブを強調 -->
          <button class="nav-link" :class="{ active: tab === 'certs' }" @click="tab = 'certs'">
            所持資格
          </button>
        </li>
        <li class="nav-item">
          <button class="nav-link" :class="{ active: tab === 'goals' }" @click="tab = 'goals'">
            目標
          </button>
        </li>
      </ul>

      <!-- 所持資格タブのコンテンツ -->
      <div v-if="tab === 'certs'">
        <div v-if="detail.certifications.length === 0" class="text-muted text-center py-4">
          所持資格はありません
        </div>
        <!-- `String(Math.random())` は cert.id が存在しない場合のフォールバック -->
        <div v-for="cert in detail.certifications" :key="cert.id || String(Math.random())" class="card shadow-sm mb-2">
          <div class="card-body py-2">
            <!-- getCertName: TypeScript 型・Supabase 生 JSON のどちらでも資格名を取得 -->
            <div class="fw-bold">{{ getCertName(cert) }}</div>
            <small class="text-muted">{{ getCertDate(cert) }}</small>
          </div>
        </div>
      </div>

      <!-- 目標タブのコンテンツ -->
      <div v-if="tab === 'goals'">
        <div v-if="detail.goals.length === 0" class="text-muted text-center py-4">
          目標はありません
        </div>
        <div v-for="goal in detail.goals" :key="goal.id || String(Math.random())" class="card shadow-sm mb-2">
          <div class="card-body py-2">
            <div class="d-flex justify-content-between">
              <span class="fw-bold">{{ getGoalName(goal) }}</span>
              <!-- StatusBadge にステータスを渡してカラーバッジを表示 -->
              <StatusBadge :status="getGoalStatus(goal)" />
            </div>
            <small class="text-muted">目標日: {{ getGoalDate(goal) }}</small>
          </div>
        </div>
      </div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
// `useRoute` で URL のパラメータや クエリを取得できる
import { useRoute } from 'vue-router'
import { useCommunityStore } from '../stores/community'
import GoodMark from '../components/community/GoodMark.vue'
import StatusBadge from '../components/goal/StatusBadge.vue'
import type { GoalStatus } from '../types'

// 現在の URL 情報（パラメータ等）
const route = useRoute()
const communityStore = useCommunityStore()

// 現在選択されているタブ（'certs' = 所持資格、'goals' = 目標）
const tab = ref<'certs' | 'goals'>('certs')

// 詳細データの参照（初期値は既にキャッシュされているデータ）
const detail = ref(communityStore.userDetail)

// ページ表示時に API からユーザー詳細を取得
onMounted(async () => {
  // `route.params.id` で URL の `:id` 部分を取得（as string で型アサーション）
  await communityStore.fetchUserDetail(route.params.id as string)
  // 取得後にストアの最新値を参照する
  detail.value = communityStore.userDetail
})

// ================================================================
// バックエンドから返ってくる「生 JSON データ」と
// TypeScript 型定義済みデータの両方に対応するヘルパー関数
// ================================================================
// バックエンドが Supabase の JSON をそのまま返すため、
// カラム名（holdt, TBL_MASTER.masnm 等）でアクセスする場合がある。
// `any` 型を使うことでどちらの形式でも動作する。
// ================================================================

/**
 * 資格名を取得する
 * TypeScript 型: cert.certification_name
 * Supabase 生 JSON: cert.TBL_MASTER.masnm
 */
function getCertName(cert: any): string {
  return cert.certification_name || cert.TBL_MASTER?.masnm || ''
}

/**
 * 取得日を取得する
 * TypeScript 型: cert.acquired_date
 * Supabase 生 JSON: cert.holdt
 */
function getCertDate(cert: any): string {
  return cert.acquired_date || cert.holdt || '日付未設定'
}

/**
 * 目標の資格名を取得する
 */
function getGoalName(goal: any): string {
  return goal.certification_name || goal.TBL_MASTER?.masnm || ''
}

/**
 * ステータスを取得する（GoalStatus 型として返す）
 * `as GoalStatus` は TypeScript の型アサーション（「この値は GoalStatus 型だ」と断言）
 */
function getGoalStatus(goal: any): GoalStatus {
  return (goal.status || goal.goast || 'exam_date') as GoalStatus
}

/**
 * 目標日を取得する
 */
function getGoalDate(goal: any): string {
  return goal.target_date || goal.goatd || ''
}
</script>
