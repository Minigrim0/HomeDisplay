<template>
    <h1 style="width: 100%;text-align: center;max-height: 5vh">HomeDisplay</h1>
    <div class="row">
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
