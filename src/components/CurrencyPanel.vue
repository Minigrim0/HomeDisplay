<template>
    <div class="panel">
        <div style="width: 100%;text-align: center;">
            <h1>🏠 Home Display 🏠</h1>
            <p class="bot-text">{{ current_date }}</p>
            <p class="top-text">{{ current_time }}</p>
        </div>
        <div class="panel-div">
            <h3 class="panel-title">
                💲 Currency 💲
                <button class="link-button" @click="load_currency_data">🔁</button>
            </h3>
            <div v-if="loading" class="ring">
                <div class="ball-holder">
                    <div class="ball"></div>
                </div>
            </div>
            <div v-else-if="error === null && currency !== {}">
                <div>
                    <p style="text-align: center;">
                        <span style="border: 2px solid whitesmoke;padding: 0.2em;border-radius: 5px;">
                            {{ currency.from_currency_amount.toFixed(2) }} {{ currency.from_currency }}
                        </span>
                        =
                        <span style="border: 2px solid whitesmoke;padding: 0.2em;border-radius: 5px;">
                            {{ currency.to_currency_amount.toFixed(2) }} {{ currency.to_currency }}
                        </span>
                    </p>
                    <small style="font-size: 0.7em;">
                        last update {{ refresh_date }}
                    </small>
                </div>
            </div>
            <div v-else>
                <p style="color: red">{{ error }}</p>
            </div>
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
            error: null,
            current_time: null,
            current_date: null,
            current_date_interval: null,
            clock_interval: null
        }
    },
    computed: {
        refresh_date() {
            let date_fetched = new Date(this.currency.timestamp * 1000);
            let date = `${this.zeroPad(date_fetched.getDate(), 2)}/${this.zeroPad(date_fetched.getMonth()+1, 2)}/${date_fetched.getFullYear()}`;
            let time = `${this.zeroPad(date_fetched.getHours(), 2)}:${this.zeroPad(date_fetched.getMinutes(), 2)}`;
            return `${date} ${time}`;
        }
    },
    methods: {
        zeroPad (num, places){
            return String(num).padStart(places, '0')
        },
        load_currency_data(){
            this.loading = true;
            invoke("get_currency")
                .then(response => this.currency = response)
                .catch(error => this.error = error)
                .finally(() => this.loading = false);
        },
        update_current_date(){
            let date = new Date();
            this.current_date = `${this.zeroPad(date.getDate(), 2)}/${this.zeroPad(date.getMonth()+1, 2)}/${date.getFullYear()}`;
        },
        update_current_time() {
            let date = new Date();
            this.current_time = `${this.zeroPad(date.getHours(), 2)}:${this.zeroPad(date.getMinutes(), 2)}:${this.zeroPad(date.getSeconds(), 2)}`;
        }
    },
    mounted(){
        this.load_currency_data();
        setInterval(() => {
            this.load_currency_data();
        }, 3600000);

        this.update_current_time();
        this.update_current_date();

        this.clock_interval = setInterval(() => {
            this.update_current_time();
        }, 1000);
        this.current_date_interval = setInterval(() => {
            this.update_current_date();
        }, 3600000);
    }
}
</script>


<style scoped>
.top-text {
    font-size: 2.5em;
    margin: 0;
    width: 100%;
}

.bot-text {
    font-size: 1.9em;
    margin: 0;
    width: 100%;
}
</style>
