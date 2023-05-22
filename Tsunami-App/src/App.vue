<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup

import { open } from "@tauri-apps/api/dialog"
import { homeDir } from "@tauri-apps/api/path"
import { invoke } from "@tauri-apps/api/tauri"

async function dialog() {
  console.log(await open({
    directory: true,
    filters: [{
      name: '.Tsunami',
      extensions: ['']
    }],
    multiple: false,
    defaultPath: await homeDir(),
    title: "Select Tsunami Project"
  }));
}

enum GlobalSettings{
  
}

invoke("load_global_config")

</script>

<template>
    
</template>