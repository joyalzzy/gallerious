<template>
    <div style="align-items: center; align-self: center; text-align: center;" class="d-flex align-items-center justify-content-center">
    <h2 style="padding-bottom: 2rem;">gallery ig</h2>
    <h2>{{ selectedTags }}</h2>
    <div class="filter">
        <template v-for="(tag, i) in tags" :key="i"> 
        <input type="checkbox" :id="tag.id.toString()" @change="updateTags($.)"/>
        <label :for="tag.id.toString()">{{tag.name}}</label>
        </template>
        
    </div>
    <lightgallery id="gallery"
        :settings="{ speed: 500, plugins: plugins }"
        :onInit="onInit"
        :onBeforeSlide="onBeforeSlide"
    >
        <a
            v-for="(item, i) in items"
            :key="i"
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
import { defineComponent, ref, type Ref } from 'vue';
import { takeCoverage } from 'v8';
import { resolveSoa } from 'dns';


console.log(import.meta.env.VITE_API_URL)
type response = {
    links: String[],
    tags: {
        id: String,
        name: String
    }[]
}
// let res : response = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3000/v1"}/links`).then(
    // res => res.data
// )
let res: response = {
    links: ['https://cdn.discordapp.com/attachments/1140180380683612160/1140564617077203047/huge_2023-08-14_16.26.39.png?ex=6551e013&is=653f6b13&hm=1ef2518ffe1ffd1742caa85dd22c800605a721d62ca0ab9c38283e4a1f615a86&', 'https://cdn.discordapp.com/attachments/1140180380683612160/1140564620046766100/huge_2023-08-14_16.24.44.png?ex=6551e014&is=653f6b14&hm=76450bbb2660afa4306198523e6b345069e990db653455f2f55b9fc52aa18432&'],
    tags: [{
        id: 'osu',
        name: 'osu'
    }, {
        id: 'val',
        name: 'val'
    }]
}






export default defineComponent({
    name: 'App',
    components: {
        // Lightgallery,
    },
    data: () => ({
        plugins: [ lgZoom, lgVideo],
        items: res.links.map((a: String, i) => {
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
        }),

        // items:  res.lin,
        tags: res.tags,
        selectedTags: selectedTags
    }),

    methods: {
        onInit: () => {
            console.log('initalised')
        },
        onBeforeSlide: () => {

        },
        tagChange: (x: String) => {

        }
    },
    
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
