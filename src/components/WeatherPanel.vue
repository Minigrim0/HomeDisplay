<template>
    <div class="panel-div">
        <h3 class="panel-title">☀️ Weather ☀️</h3>
        <div v-if="loading">loading...</div>
        <div v-else-if="error === null && weather !== {}">
            <div>
                <p>🌡️ {{ weather.main.temp.toFixed(2) }} C</p>
                <p>👉👈 {{ weather.main.feels_like.toFixed(2) }} C</p>
                <p>🥶 {{ weather.main.temp_min.toFixed(2) }} C</p>
                <p>🥵 {{ weather.main.temp_max.toFixed(2) }} C</p>
            </div>
            <div>
                weather: {{ weather.weather[0].description }}
                <img :src="`src/assets/img/owm/icons/${this.weather.weather[0].icon}@2x.png`" alt="weather icon"/>
            </div>
            <small>
                {{ sunrise }} => {{ sunset }}
            </small>
        </div>
        <div v-else>
            <p style="color: red">{{ error }}</p>
        </div>
        <button @click="load_weather_data">refresh</button>
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
