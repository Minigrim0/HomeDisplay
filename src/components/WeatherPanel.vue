<template>
    <div class="panel-div">
        <h3 class="panel-title">
            ☀️ Weather ☀️
            <button class="link-button" @click="load_weather_data">🔁</button>
        </h3>
        <div v-if="loading" class="ring">
            <div class="ball-holder">
                <div class="ball"></div>
            </div>
        </div>
        <div v-else-if="error === null && weather !== {}">
            <h3 style="width: 100%;text-align: center;border-top: 1px solid white;padding-top: 5px">🌡️ Temperature 🌡️</h3>
            <div>
                <p style="margin-top: 5px;margin-bottom: 5px;font-size: 3em;text-align: center">{{ weather.main.temp.toFixed(0) }}°C</p>
                <div style="font-size: 0.8em;text-align: center" class="row">
                    <p style="max-width: 33%;min-width: 30%;border-right: 1px solid white;">Feel {{ weather.main.feels_like.toFixed(0) }}°C</p>
                    <p style="max-width: 33%;min-width: 30%;border-right: 1px solid white;">⬇️ {{ weather.main.temp_min.toFixed(0) }}°C </p>
                    <p style="max-width: 33%;min-width: 30%;"> ⬆️ {{ weather.main.temp_max.toFixed(0) }}°C</p>
                </div>
            </div>
            <h3 style="width: 100%;text-align: center;border-top: 1px solid white;padding-top: 5px">☀️ Weather ☀️</h3>
            <div style="text-align: center;width: 100%;">
                <img style="margin-top: 5px;margin-bottom: 5px;" :src="`src/assets/img/owm/icons/${this.weather.weather[0].icon}@2x.png`" alt="weather icon"/>
                <p>{{ weather.weather[0].description }}</p>
            </div>
            <h3 style="width: 100%;text-align: center;border-top: 1px solid white;padding-top: 5px">🌕 Day time ☀️</h3>
            <div style="text-align: center;width: 100%;">
                <p>Rise {{ sunrise }}</p>
                <p>Set {{ sunset }}</p>
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
    name: "WeatherPanel",
    data(){
        return {
            weather: {},
            loading: true,
            error: null
        }
    },
    computed: {
        sunrise() {
            let sunrise_time = new Date(this.weather.sys.sunrise * 1000)
            return `${sunrise_time.getHours()}:${sunrise_time.getMinutes()}`;
        },
        sunset() {
            let sunset_time = new Date(this.weather.sys.sunset * 1000);
            return `${sunset_time.getHours()}:${sunset_time.getMinutes()}`;
        }
    },
    methods: {
        load_weather_data(){
            this.loading = true;
            console.log("Fecthing weather data");
            invoke("get_weather")
                .then(response => this.weather = response)
                .catch(error => this.error = error)
                .finally(() => this.loading = false);
        }
    },
    mounted(){
        this.load_weather_data();
    }
}
</script>
