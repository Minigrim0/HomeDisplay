<template>
    <div v-if="error === null && currency !== {}">
        <p>
            {{ currency.from_currency_amount }} {{ currency.from_currency }} => {{ currency.to_currency_amount }} {{ currency.to_currency }}
        </p>
        <small>
            {{ currency.timestamp }}
        </small>
    </div>
    <div v-else>
        <p style="color: red">{{ error }}</p>
    </div>
    <button @click="load_currency_data">refresh</button>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "CurrencyPanel",
    data(){
        return {
            currency: {},
            loading: true,
            error: null
        }
    },
    methods: {
        load_currency_data(){
            console.log("Fecthing currency data");
            invoke("get_currency")
                .then(response => this.currency = response)
                .catch(error => this.error = error)
                .finally(() => this.loading = false);
        }
    },
    mounted(){
        this.load_currency_data();
    }
}
</script>
