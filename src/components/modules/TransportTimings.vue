<template>
    <h3>{{  transport_type }}</h3>
    <div v-for="transport in departures" :key="transport.DisplayTime">
        {{ transport.LineNumber }} - {{ transport.Destination }} - {{ transport.DisplayTime }}
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
        transport_type: {
            type: String,
            default: () => "unknown"
        }
    }
}
</script>