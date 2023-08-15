<template>
  <div class="container">
    <h1>Home Display</h1>

    <CurrencyPanel />
    <WeatherPanel />
  </div>
</template>

<script>
import CurrencyPanel from "./components/CurrencyPanel.vue";
import WeatherPanel from "./components/WeatherPanel.vue";

export default {
    name: "App",
    components: {
        CurrencyPanel,
        WeatherPanel
    }
}
</script>


<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
