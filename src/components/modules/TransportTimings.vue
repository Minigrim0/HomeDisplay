<template>
    <div v-for="mode in transport_modes" :key="mode">
        <h4>{{  mode }}</h4>
        <div v-for="departure in departures.filter(dep => dep.line.transport_mode == mode)" :key="`${departure.line.id}-${departure.display}`">
            {{ departure.line.id }} - {{ departure.destination }} - {{ departure.display }}
        </div>
    </div>
</template>

<script>
export default {
    name: "TransportTimings",
    props: {
        departures: {
            type: Array,
            required: true
        },
    },
    computed: {
        transport_modes() {
            let modes = [];
            for (let departure of this.departures) {
                if (!modes.includes(departure.line.transport_mode)) {
                    modes.push(departure.line.transport_mode);
                }
            }
            return modes;
        }
    }
}
</script>