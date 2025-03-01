<template>
  <router-view />
</template>

<script lang="ts" setup></script>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
  overflow: hidden; /* Prevent scrolling */
}

#app {
  display: flex;
  flex-direction: column;
}
</style>
