<template>
    <div style="align-items: center; align-self: center; text-align: center;" class="d-flex align-items-center justify-content-center">
    <h2 style="padding-bottom: 2rem;">gallery ig</h2>
    <lightgallery id="gallery"
        :settings="{ speed: 500, plugins: plugins }"
        :onInit="onInit"
        :onBeforeSlide="onBeforeSlide"
    >
        <a
            v-for="item in items"
            :key="item.id"
            :data-lg-size="200"
            className="gallery-item"
            :data-src="item.src"
        >
            <img v-if="!item.video" className="img-responsive"  :src="item.src.toString()" style="align-self: center; width: 100%;" />
            <video v-else className="" style="align-self: center; width: 100%;" controls>
                <source :src="item.src.toString()" type="video/mp4">
            </video>
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
import { defineComponent } from 'vue';


console.log(import.meta.env.VITE_API_URL)
const links : String[] = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3000/v1"}/links`).then(
    res => res.data
).then(
    res => {
        return res as String[]
    }
)

export default defineComponent({
    name: 'App',
    components: {
        // Lightgallery,
    },
    data: () => ({
        plugins: [ lgZoom, lgVideo],
        items: links.map((a: String, i) => {
            if (a.includes("mp4")) {
                return {
                    id: i,
                    src: a,
                    video: true
                }
            }
            else {
                return {
                    id: i,
                    src: a,
                    video: false
                }
            }
        })

    }),

    methods: {
        onInit: () => {
            console.log('initalised')
        },
        onBeforeSlide: () => {

        }
    }
    
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
