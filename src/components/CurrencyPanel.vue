<template>
    <div class="panel-div">
        <h3 class="panel-title">Currency</h3>
        <div v-if="loading">loading...</div>
        <div v-else-if="error === null && currency !== {}">
            <div>
                <p>
                    <span style="border: 2px solid whitesmoke;padding: 0.2em;border-radius: 5px;">
                        {{ currency.from_currency_amount.toFixed(2) }} {{ currency.from_currency }}
                    </span>
                    =
                    <span style="border: 2px solid whitesmoke;padding: 0.2em;border-radius: 5px;">
                        {{ currency.to_currency_amount.toFixed(2) }} {{ currency.to_currency }}
                    </span>
                </p>
                <small>
                    {{ refresh_date }}
                </small>
            </div>
            <div>
                <button @click="load_currency_data">🔁</button>
            </div>
        </div>
        <div v-else>
            <p style="color: red">{{ error }}</p>
        </div>
    </div>
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
    computed: {
        refresh_date() {
            let date_fetched = new Date(this.currency.timestamp * 1000);
            let date = `${this.zeroPad(date_fetched.getDay(), 2)}/${this.zeroPad(date_fetched.getMonth()+1, 2)}/${date_fetched.getFullYear()}`;
            let time = `${this.zeroPad(date_fetched.getHours(), 2)}:${this.zeroPad(date_fetched.getMinutes(), 2)}`;
            return `${date} ${time}`;
        }
    },
    methods: {
        zeroPad (num, places){
            return String(num).padStart(places, '0')
        },
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
