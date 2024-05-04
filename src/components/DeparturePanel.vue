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
        <div v-else-if="error === null">
            <div v-for="site in sites" :key="site.name">
                <h3>{{ site.name }}</h3>
                <div v-if="site_errors[site.id] !== undefined" style="color: red">
                    {{ site_errors[site.id] }}
                </div>
                <div v-else>
                    <TransportTimings :departures="departures[site.id]" v-if="departures[site.id].length > 0" />
                </div>
            </div>
            <small>{{ time_since_last_update }} seconds ago. <span v-if="refreshing">refreshing...</span></small>
        </div>
        <div v-else>
            <p style="color: red">{{ error }}</p>
        </div>
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
            sites: [],
            departures: {},
            site_errors: {},
            last_update: new Date(),
            time_since_last_update: 0,
            loading: true,
            refreshing: false,
            error: null
        };
    },
    methods: {
        fetch_sites() {
            this.loading = true;
            return invoke("get_sites")
                .then(response => {
                    this.sites = response;
                })
                .catch(error => {
                    this.error = error;
                });
        },
        async fetch_departures() {
            for(let site of this.sites) {
                this.departures[site.id] = {};
                await invoke("get_departures", { site: site })
                    .then(response => {
                        this.departures[site.id] = response;
                    })
                    .catch(error => {
                        this.site_errors[site.id] = error;
                    });
            }
            if (this.sites.length === 0 && this.error === null) {
                this.error = "No sites where found. Please use environment variables to set the sites.";
            }
            this.refreshing = false;
            this.last_update = new Date();
            this.loading = false;
        },
        refresh() {
            this.refreshing = true;
            this.fetch_sites().then(() => this.fetch_departures());
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
        this.fetch_sites().then(() => this.fetch_departures());
        this.upd_timer(this.dateDiffInDays(this.last_update, new Date()));
        setInterval(this.refresh, 60000);  // Refresh the data every minute
    }
}
</script>
