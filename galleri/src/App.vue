<template>
    <div style="align-items: center; align-self: center; text-align: center;" class="d-flex align-items-center justify-content-center">
    <h2 style="padding-bottom: 2rem;">gallery ig</h2>
    <lightgallery
        :settings="{ speed: 500, plugins: plugins }"
    >
        <a
            v-for="item in items"
            :key="item.id"
            :data-lg-size="200"
            className="gallery-item"
            :data-src="item.src"
        >
            <img className="img-responsive"  :src="item.src" width="500" />
        </a>
        
        
    </lightgallery>
    </div>
</template>

<script lang="ts">
import Lightgallery from 'lightgallery/vue';
import lgThumbnail from 'lightgallery/plugins/thumbnail';
import lgZoom from 'lightgallery/plugins/zoom';
import lgVideo from 'lightgallery/plugins/video'
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
        plugins: [lgThumbnail, lgZoom, lgVideo],
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
            id: 1,
            src: "https://media.discordapp.net/attachments/1140180380683612160/1140180381652484147/huge_2023-08-13_15.09.15.png?ex=65473fba&is=6534caba&hm=63bca0e9e8b1b3b838280ec6fb16626f1928c5f78e0916b9e35d3a080dad9141&=&width=1058&height=595"
        },{
            id: 2,
            src: "https://cdn.discordapp.com/attachments/1139455056786231346/1139455058371674152/a3k-converted.mp4?ex=65449c37&is=65322737&hm=c41f95aa00316fe651e777ca05fc943362d150fd857882a65324d309c0fe292e&"
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
