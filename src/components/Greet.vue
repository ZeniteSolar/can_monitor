<script setup>
import { ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

const output = ref("");
const outputs = ref([]);
const inputs = ref([]);

await listen('can_message', (event) => {
  let input = event.payload
  inputs.value.push({ timestamp: Date.now(), message: input })
})
</script>

<template>
  <div style="display: grid; grid-template-columns: auto auto;">
    <div style="grid-column: span 2; grid-row: 1;">
      <label for="input" style="display: block;">Message</label>
      <input id="input" v-model="output">
      <br>
      <button @click="sendOutput()">Send to Rust</button>
    </div>
    <div style="grid-column: 1; grid-row: 2;">
      <h3>front2back events</h3>
      <ol>
        <li v-for="output in outputs">
          {{output}}
        </li>
      </ol>
    </div>
    <div style="grid-column: 2; grid-row: 2;">
      <h3>Can Messages</h3>
      <ol>
        <li v-for="input in inputs">
          {{input}}
        </li>
      </ol>
    </div>
  </div>
</template>