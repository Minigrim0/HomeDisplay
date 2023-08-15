<template>
    <div v-if="loading">loading...</div>
    <div v-else-if="error === null && weather !== {}">
        <p>
            temp {{ weather.main.temp.toFixed(2) }} C<br/>
            feels like {{ weather.main.feels_like.toFixed(2) }} C<br/>
            min {{ weather.main.temp_min.toFixed(2) }} C<br/>
            max {{ weather.main.temp_max.toFixed(2) }} C<br/>

            weather: {{ weather.weather[0].description }}
            <img :src="`src/assets/img/owm/icons/${this.weather.weather[0].icon}@2x.png`" alt="weather icon"/>
        </p>
        <small>
            {{ weather.timestamp }}
        </small>
    </div>
    <div v-else>
        <p style="color: red">{{ error }}</p>
    </div>
    <button @click="load_weather_data">refresh</button>
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
    methods: {
        load_weather_data(){
            console.log("Fecthing weather data");
            invoke("get_weather")
                .then(response => this.weather = response)
                .then(() => console.log(this.weather))
                .catch(error => this.error = error)
                .finally(() => this.loading = false);
        }
    },
    mounted(){
        this.load_weather_data();
    }
}
</script>
