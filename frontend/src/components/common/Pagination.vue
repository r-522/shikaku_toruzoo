<template>
  <nav v-if="totalPages > 1">
    <ul class="pagination justify-content-center">
      <li class="page-item" :class="{ disabled: currentPage <= 1 }">
        <a class="page-link" href="#" @click.prevent="$emit('pageChange', currentPage - 1)">
          &laquo;
        </a>
      </li>
      <li
        v-for="page in totalPages"
        :key="page"
        class="page-item"
        :class="{ active: page === currentPage }"
      >
        <a class="page-link" href="#" @click.prevent="$emit('pageChange', page)">
          {{ page }}
        </a>
      </li>
      <li class="page-item" :class="{ disabled: currentPage >= totalPages }">
        <a class="page-link" href="#" @click.prevent="$emit('pageChange', currentPage + 1)">
          &raquo;
        </a>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  total: number
  perPage: number
  currentPage: number
}>()

defineEmits<{
  pageChange: [page: number]
}>()

const totalPages = computed(() => Math.ceil(props.total / props.perPage))
</script>
