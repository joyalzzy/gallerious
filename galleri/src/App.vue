<template>
    <h3>gallery ig</h3>
    <lightgallery
        :settings="{ speed: 500, plugins: plugins }"
    >
        
        
    </lightgallery>
</template>

<script lang="ts">
import Lightgallery from 'lightgallery/vue';
import lgThumbnail from 'lightgallery/plugins/thumbnail';
import lgZoom from 'lightgallery/plugins/zoom';
import axios from 'axios';
import { stringifyQuery } from 'vue-router';
import { defineComponent } from 'vue';
import $ from 'jquery';


console.log(import.meta.env.VITE_API_URL)
// const links = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3000/v1"}/links`).then(
    // res => res.data
// ).then(
    // res => {
        // return res as String[]
    // }
// )
export default defineComponent({
    name: 'App',
    components: {
    },
    data: () => ({
        plugins: [lgThumbnail, lgZoom],
        // items: links.map((a, i) => {
            // if (!a.includes("mp4")) {
                // return {
                    // src: a,
                    // scrt: a
                // }
            // }
            // else {
                // return {
                    // src: a,
                    // scrt: 'video/mp4'
                // }
            // }
        // })
        items: [{
            src: "https://media.discordapp.net/attachments/1140180380683612160/1140180381652484147/huge_2023-08-13_15.09.15.png?ex=65473fba&is=6534caba&hm=63bca0e9e8b1b3b838280ec6fb16626f1928c5f78e0916b9e35d3a080dad9141&=&width=1058&height=595"
        }]
    })
});
</script>
<style lang="css">
@import url('https://cdn.jsdelivr.net/npm/lightgallery@2.1.0-beta.1/css/lightgallery.css');
@import url('https://cdn.jsdelivr.net/npm/lightgallery@2.1.0-beta.1/css/lg-zoom.css');

body {
    margin: 0;
}

.gallery-item {
    margin: 5px
}
</style>
