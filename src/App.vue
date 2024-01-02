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
