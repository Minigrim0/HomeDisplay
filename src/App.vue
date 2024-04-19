<template>
    <div class="container">
        <WeatherPanel />
        <CurrencyPanel />
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
    },
}
</script>
