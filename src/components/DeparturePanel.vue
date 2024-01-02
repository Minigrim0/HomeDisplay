<template>
    <div class="panel panel-div">
        <h3 class="panel-title">
            🚂 Departures 🚂
            <button class="link-button" @click="refresh">🔁</button>
        </h3>
        <div v-if="loading" class="ring">
            <div class="ball-holder">
                <div class="ball"></div>
            </div>
        </div>
        <div v-else-if="error === null && departures !== {}">
            <div v-for="departure_info in departures" :key="departure_info.stop.Name">
                <h4>{{ departure_info.stop.Name }}</h4>
                <TransportTimings :departures="departure_info.departures.Metros" transport_type="Metro" v-if="departure_info.departures.Metros.length > 0" />
                <TransportTimings :departures="departure_info.departures.Buses" transport_type="Bus" v-if="departure_info.departures.Buses.length > 0" />
                <TransportTimings :departures="departure_info.departures.Ships" transport_type="Ship" v-if="departure_info.departures.Ships.length > 0" />
                <TransportTimings :departures="departure_info.departures.Trams" transport_type="Tram" v-if="departure_info.departures.Trams.length > 0" />
                <TransportTimings :departures="departure_info.departures.Trains" transport_type="Train" v-if="departure_info.departures.Trains.length > 0" />
                <div v-if="no_timings(departure_info)" style="color: red">
                    No timings could be fetched 😥
                </div>
            </div>
        </div>
        <small>{{ time_since_last_update }} seconds ago. <span v-if="refreshing">refreshing...</span></small>
    </div>
</template>

<script>
import { invoke } from '@tauri-apps/api';

import TransportTimings from './modules/TransportTimings.vue';

export default {
    name: "DeparturePanel",
    components: {
        TransportTimings
    },
    data() {
        return {
            departures: {},
            last_update: new Date(),
            time_since_last_update: 0,
            loading: true,
            refreshing: false,
            error: null
        };
    },
    methods: {
        fetch_departures() {
            invoke("get_departures")
                .then(response => {
                    this.departures = response;
                    this.last_update = new Date();
                })
                .catch(error => {
                    this.error = error;
                    console.log("Error !", error)
                })
                .finally(() => {
                    this.loading = false
                    this.refreshing = false;
                });
        },
        refresh() {
            this.refreshing = true;
            this.fetch_departures();
        },
        no_timings(departure_info) {
            return (
                departure_info.departures.Metros.length === 0
             ) && (
                departure_info.departures.Trams.length === 0
              ) && (
                departure_info.departures.Buses.length === 0
              ) && (
                departure_info.departures.Trains.length === 0
              ) && (
                departure_info.departures.Ships.length === 0
              );
        },
        dateDiffInDays(a, b) {
            return b - a;
        },
        upd_timer(diff){
            this.time_since_last_update = Math.floor(diff / 1000 % 60);
            setTimeout(() => this.upd_timer(this.dateDiffInDays(this.last_update, new Date())), 1000);
        },
    },
    mounted() {
        this.fetch_departures();
        this.upd_timer(this.dateDiffInDays(this.last_update, new Date()));
        setInterval(this.refresh, 60000);  // Refresh the data every minute
    }
}
</script>