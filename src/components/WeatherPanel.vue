<template>
    <div class="panel panel-div">
        <h3 class="panel-title">
            ☀️ Weather ☀️
            <button class="link-button" @click="load_weather_data">🔁</button>
        </h3>
        <div v-if="loading" class="ring">
            <div class="ball-holder">
                <div class="ball"></div>
            </div>
        </div>
        <div v-else-if="error === null">
            <h3 class="section-separator-title">🌡️ Temperature 🌡️</h3>
            <div>
                <p class="central-content">{{ weather.main.temp.toFixed(0) }}°C</p>
                <div class="small-grid">
                    <p class="small-grid-elem">Feel {{ weather.main.feels_like.toFixed(0) }}°C</p>
                    <p class="small-grid-elem center">⬇️ {{ weather.main.temp_min.toFixed(0) }}°C </p>
                    <p class="small-grid-elem"> ⬆️ {{ weather.main.temp_max.toFixed(0) }}°C</p>
                </div>
            </div>
            <h3 class="section-separator-title">☀️ Weather ☀️</h3>
            <div style="text-align: center;width: 100%;">
                <img
                    class="central-content"
                    style="max-height: 64px;"
                    :src="`src/assets/img/owm/icons/${this.weather.weather[0].icon}@2x.png`"
                    alt="weather icon"
                />
                <p>{{ weather.weather[0].description }}</p>
            </div>
            <h3 class="section-separator-title">🌕 Day time ☀️</h3>
            <div style="text-align: center;width: 100%;">
                <p>🌅 {{ sunrise }} 🌄 {{ sunset }}</p>
            </div>
            <small style="font-size: 0.7em;">
                {{ time_since_last_update }} minute{{ time_since_last_update > 1 ? 's' : '' }} ago.
            </small>
        </div>
        <div v-else>
            <p style="color: red">{{ error }}</p>
        </div>
    </div>
</template>

<script>
import { invoke } from "@tauri-apps/api/tauri";

export default {
    name: "WeatherPanel",
    data(){
        return {
            weather: {},
            loading: true,
            error: null,
            last_update: new Date(),
            time_since_last_update: 0
        }
    },
    computed: {
        sunrise() {
            let sunrise_time = new Date(this.weather.sys.sunrise * 1000)
            return `${this.zeroPad(sunrise_time.getHours(), 2)}:${this.zeroPad(sunrise_time.getMinutes())}`;
        },
        sunset() {
            let sunset_time = new Date(this.weather.sys.sunset * 1000);
            return `${this.zeroPad(sunset_time.getHours(), 2)}:${this.zeroPad(sunset_time.getMinutes(), 2)}`;
        }
    },
    methods: {
        load_weather_data(){
            this.loading = true;
            console.log("Fecthing weather data");
            invoke("get_weather")
                .then(response => this.weather = response)
                .catch(error => this.error = error)
                .finally(() => {
                    this.loading = false
                    this.last_update = new Date();
                });
        },
        upd_timer(diff){
            this.time_since_last_update = Math.floor(diff / 60000);
        },
        dateDiffInDays(a, b) {
            return b - a;
        },
        zeroPad (num, places){
            return String(num).padStart(places, '0')
        },
    },
    mounted(){
        this.load_weather_data();
        setInterval(() => {
            this.upd_timer(this.dateDiffInDays(this.last_update, new Date()));
        }, 1000);
        setInterval(() => {
            this.load_weather_data();
        }, 1800000);
    }
}
</script>
