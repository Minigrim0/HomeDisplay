<template>
    <p v-if="error === null">
        {{ base_amount }} {{ base_currency }} => {{ target_amount }} {{ target_currency }}
    </p>
    <p v-else style="color: red">
        {{ error }}
    </p>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "CurrencyPanel",
    data(){
        return {
            base_currency: "EUR",
            base_amount: 1,
            target_currency: "SEK",
            target_amount: 0.09,
            loading: true,
            error: null
        }
    },
    methods: {
        async load_currency_data(){
            invoke("get_currency")
                .then(response => console.log(response))
                .catch(error => this.error = error)
                .finally(() => this.loading = false);
        }
    },
    mounted(){
        this.load_currency_data();
    }
}
</script>
