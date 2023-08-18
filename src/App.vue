<template>
  <div class="container">
    <h1>Home Display</h1>

    <CurrencyPanel />
    <WeatherPanel />
    <DeparturePanel />
  </div>
</template>

<script>
import CurrencyPanel from "./components/CurrencyPanel.vue";
import WeatherPanel from "./components/WeatherPanel.vue";
import DeparturePanel from "./components/DeparturePanel.vue";

export default {
    name: "App",
    components: {
        CurrencyPanel,
        WeatherPanel,
        DeparturePanel
    }
}
</script>
