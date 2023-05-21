<script setup lang="ts">
  import TabButton from './components/TabButton.vue';
  import Viewport from './components/Viewport.vue';
</script>

<template>
  <div>
    <button class="tab-button" @click="component='Constants'">Constants</button>
    <button class="tab-button" @click="component='Hardware'">Hardware</button>
    <button class="tab-button" @click="component='Telemetry'">Telemetry</button>
  </div>
    <div id="tab-window">
      <KeepAlive> 
        <component :is="component" />
      </KeepAlive>
    </div>

</template>

<script lang="ts">
import Constants from './components/tabs/Constants.vue';
import Hardware from './components/tabs/Hardware.vue';
import Telemetry from './components/tabs/Telemetry.vue';

export default {
  name: "tab-window",
  components: {
    Constants,
    Hardware,
    Telemetry
  },
  data() {
    return {
      component: "Constants",
    };
  },
};

</script>