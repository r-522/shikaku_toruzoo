<template>
  <div class="position-relative">
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
    <ul
      v-if="suggestions.length > 0 && showSuggestions"
      class="list-group position-absolute w-100 shadow"
      style="z-index: 1000; max-height: 250px; overflow-y: auto"
    >
      <li
        v-for="(item, index) in suggestions"
        :key="item.id"
        class="list-group-item list-group-item-action"
        :class="{ active: index === activeIndex }"
        @click="selectItem(item)"
        @mouseenter="activeIndex = index"
      >
        <span>{{ item.name }}</span>
        <small class="text-muted ms-2">{{ item.category }}</small>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { masterApi } from '../../api/master'
import type { MasterCertification } from '../../types'

const props = defineProps<{
  modelValue: string
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
  select: [payload: { name: string; master_id: string | null }]
}>()

const suggestions = ref<MasterCertification[]>([])
const showSuggestions = ref(false)
const activeIndex = ref(-1)
let debounceTimer: ReturnType<typeof setTimeout> | null = null

function onInput(e: Event) {
  const value = (e.target as HTMLInputElement).value
  emit('update:modelValue', value)
  debouncedSearch(value)
}

function debouncedSearch(query: string) {
  if (debounceTimer) clearTimeout(debounceTimer)
  if (query.length < 2) {
    suggestions.value = []
    showSuggestions.value = false
    return
  }
  debounceTimer = setTimeout(async () => {
    try {
      const { data } = await masterApi.search(query)
      suggestions.value = data.certifications
      showSuggestions.value = suggestions.value.length > 0
      activeIndex.value = -1
    } catch {
      suggestions.value = []
    }
  }, 300)
}

function selectItem(item: MasterCertification) {
  emit('update:modelValue', item.name)
  emit('select', { name: item.name, master_id: item.id })
  closeSuggestions()
}

function moveDown() {
  if (activeIndex.value < suggestions.value.length - 1) {
    activeIndex.value++
  }
}

function moveUp() {
  if (activeIndex.value > 0) {
    activeIndex.value--
  }
}

function selectCurrent() {
  if (activeIndex.value >= 0 && activeIndex.value < suggestions.value.length) {
    selectItem(suggestions.value[activeIndex.value])
  } else {
    emit('select', { name: props.modelValue, master_id: null })
    closeSuggestions()
  }
}

function closeSuggestions() {
  showSuggestions.value = false
  activeIndex.value = -1
}
</script>
