<!-- ============================================================
components/master/CertAutocomplete.vue — 資格名オートコンプリートコンポーネント
============================================================
このファイルは資格名の入力フォームに補完候補を表示するコンポーネント。

【動作の流れ】
1. ユーザーが入力 → onInput() でデバウンスサーチを起動
2. 300ms 後に masterApi.search() を呼び出し
3. 候補が返ってきたら ul タグで一覧表示
4. 候補を選択 → select イベントで親に { name, master_id } を通知

【v-model との連携（update:modelValue パターン）】
Vue 3 のカスタムコンポーネントで v-model を使うためのパターン。
- props の `modelValue` で親からの値を受け取る
- `emit('update:modelValue', 新しい値)` で親に値の更新を通知
- 親は `v-model="variableName"` と書ける

【キーボード操作対応】
↓キー: 候補リストを下に移動（moveDown）
↑キー: 候補リストを上に移動（moveUp）
Enter: 現在選択中の候補を確定（selectCurrent）
Escape: 候補リストを閉じる（closeSuggestions）
============================================================ -->
<template>
  <!-- position-relative: 候補リスト（position-absolute）の基準になる -->
  <div class="position-relative">
    <!-- テキスト入力フォーム -->
    <input
      type="text"
      class="form-control"
      :value="modelValue"
      @input="onInput"
      @keydown.down.prevent="moveDown"
      @keydown.up.prevent="moveUp"
      @keydown.enter.prevent="selectCurrent"
      @keydown.escape="closeSuggestions"
      placeholder="資格名を入力..."
    />

    <!-- 候補リスト（候補があり、かつ showSuggestions が true の場合のみ表示） -->
    <ul
      v-if="suggestions.length > 0 && showSuggestions"
      class="list-group position-absolute w-100 shadow"
      style="z-index: 1000; max-height: 250px; overflow-y: auto"
    >
      <!-- 各候補アイテム -->
      <!-- :class="{ active: index === activeIndex }" でキーボードで選択中の項目を強調 -->
      <li
        v-for="(item, index) in suggestions"
        :key="item.id"
        class="list-group-item list-group-item-action"
        :class="{ active: index === activeIndex }"
        @click="selectItem(item)"
        @mouseenter="activeIndex = index"
      >
        <span>{{ item.name }}</span>
        <!-- カテゴリをグレーで小さく表示 -->
        <small class="text-muted ms-2">{{ item.category }}</small>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { masterApi } from '../../api/master'
import type { MasterCertification } from '../../types'

// ---- Props と Emits の定義 ----
const props = defineProps<{
  modelValue: string  // v-model で親から渡される現在の入力値
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]                          // v-model の更新通知
  select: [payload: { name: string; master_id: string | null }] // 候補選択時の通知
}>()

// ---- リアクティブな状態 ----
const suggestions = ref<MasterCertification[]>([])  // 候補リスト
const showSuggestions = ref(false)                  // 候補リストの表示/非表示
const activeIndex = ref(-1)                         // キーボードで選択中のインデックス（-1 = なし）
// デバウンスタイマーの参照（内部実装）
let debounceTimer: ReturnType<typeof setTimeout> | null = null

// ---- イベントハンドラ ----

/**
 * 入力イベントハンドラ
 * v-model の更新通知とデバウンスサーチの起動
 */
function onInput(e: Event) {
  // `e.target as HTMLInputElement` は型アサーション: target を input 要素として扱う
  const value = (e.target as HTMLInputElement).value
  // 親コンポーネントに入力値の変更を通知（v-model の仕組み）
  emit('update:modelValue', value)
  debouncedSearch(value)
}

/**
 * デバウンス付きの検索（連続入力時の API 呼び出し回数を抑制）
 */
function debouncedSearch(query: string) {
  // 前回のタイマーをキャンセル
  if (debounceTimer) clearTimeout(debounceTimer)
  // 2 文字未満は検索しない（候補をクリアして非表示）
  if (query.length < 2) {
    suggestions.value = []
    showSuggestions.value = false
    return
  }
  // 300ms 後に API を呼ぶ（入力が止まってから検索する）
  debounceTimer = setTimeout(async () => {
    try {
      const { data } = await masterApi.search(query)
      suggestions.value = data.certifications
      // 候補がある場合のみ表示する
      showSuggestions.value = suggestions.value.length > 0
      // キーボード選択をリセット
      activeIndex.value = -1
    } catch {
      suggestions.value = []
    }
  }, 300)
}

/**
 * 候補アイテムを選択する
 *
 * @param item - 選択されたマスタ資格データ
 */
function selectItem(item: MasterCertification) {
  // 入力値を選択した資格名に更新
  emit('update:modelValue', item.name)
  // 親に選択結果を通知（name + master_id）
  emit('select', { name: item.name, master_id: item.id })
  closeSuggestions()
}

/** キーボードの↓で候補リストを下に移動 */
function moveDown() {
  // 最後の候補を超えないように制限
  if (activeIndex.value < suggestions.value.length - 1) {
    activeIndex.value++
  }
}

/** キーボードの↑で候補リストを上に移動 */
function moveUp() {
  // 最初の候補（0）より上に行かないように制限
  if (activeIndex.value > 0) {
    activeIndex.value--
  }
}

/**
 * Enter キーで現在選択中の候補を確定する
 *
 * キーボードで候補が選択されていない場合は手入力の値をそのまま使う。
 */
function selectCurrent() {
  if (activeIndex.value >= 0 && activeIndex.value < suggestions.value.length) {
    // キーボードで候補を選択している場合
    selectItem(suggestions.value[activeIndex.value])
  } else {
    // 候補を選択していない場合: 手入力の値で確定（master_id は null）
    emit('select', { name: props.modelValue, master_id: null })
    closeSuggestions()
  }
}

/** 候補リストを閉じてインデックスをリセット */
function closeSuggestions() {
  showSuggestions.value = false
  activeIndex.value = -1
}
</script>
