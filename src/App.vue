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

import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "App",
    components: {
        CurrencyPanel,
        WeatherPanel,
        DeparturePanel
    },
    methods: {
        update_apis() {
            // Invoke the Rust API to fetch the APIs
            invoke("fetch_apis");
            setTimeout(() => {
                invoke("fetch_apis");
                this.update_apis();
                console.log("Updated APIs");
            }, 3600000);
        }
    },
    mounted() {
        this.update_apis();
    }
}
</script>
