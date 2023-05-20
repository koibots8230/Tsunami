<script setup lang="ts">
  import TabButton from './components/TabButton.vue';
  import Viewport from './components/Viewport.vue';
</script>

<template>
  <header>
    <TabButton :title='Constants' :run="component='Constants'"></TabButton>
    <TabButton :title='Hardware' :run="component='Hardware'"></TabButton>
    <TabButton :title='Telemetry' :run="component='Telemetry'"></TabButton>
  </header>
  
  <main>
    <div id="tab-window">
      <KeepAlive> 
        <component :is="component" />
      </KeepAlive>
    </div>
  </main>

  <aside>
    <Viewport></Viewport>
  </aside>
  <footer>

  </footer>
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