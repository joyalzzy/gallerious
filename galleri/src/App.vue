<template>
    <button v-on:click="updateSlides">Add new image</button>
    <lightgallery :settings="{ speed: 500, plugins: plugins }" :onInit="onInit" :onBeforeSlide="onBeforeSlide">
        <a v-for="item in items" :key="item.id"  className="gallery-item" :data-src="item.src">
              <!-- @vue-ignore -->
            <img className="img-responsive" :src="item.thumb" />
        </a>
    </lightgallery>
</template>

<script lang="ts">
import Lightgallery from 'lightgallery/vue';
import lgZoom from 'lightgallery/plugins/zoom';
import axios from 'axios';
import { stringifyQuery } from 'vue-router';
import { defineComponent } from 'vue';


let lightGallery: any = null;
console.log(import.meta.env.VITE_API_URL)
const links = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3000/v1"}/links`).then(
    res => res.data
).then(
    res => {
        return res as String[]
    }
)
export default defineComponent({
    name: 'App',
    components: {
        Lightgallery,
    },
    watch: {
        items(newVal, oldVal) {
            this.$nextTick(() => {
                lightGallery.refresh();
            });
        },
    },
    data: () => ({
        plugins: [lgZoom],
        items: links.map((a, i) => {
            return {
                id: (i + 2).toString(),
                src: a,
                thumb: a
            }
        })
        // [
        // {
        // id: '1',
        // size: '1400-933',
        // src:
        // 'https://images.unsplash.com/photo-1542103749-8ef59b94f47e?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1400&q=80',
        // thumb:
        // 'https://images.unsplash.com/photo-1542103749-8ef59b94f47e?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=240&q=80',
        // subHtml: `<div class="lightGallery-captions">
        // <h4>Photo by <a href="https://unsplash.com/@dann">Dan</a></h4>
        // <p>Published on November 13, 2018</p>
        // </div>`,
        // },
        // {
        // id: '2',
        // size: '1400-933',
        // src:
        // 'https://images.unsplash.com/photo-1473876988266-ca0860a443b8?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1400&q=80',
        // thumb:
        // 'https://images.unsplash.com/photo-1473876988266-ca0860a443b8?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=240&q=80',
        // subHtml: `<div class="lightGallery-captions">
        // <h4>Photo by <a href="https://unsplash.com/@kylepyt">Kyle Peyton</a></h4>
        // <p>Published on September 14, 2016</p>
        // </div>`,
        // },
        // {
        // id: '3',
        // size: '1400-932',
        // src:
        // 'https://images.unsplash.com/photo-1588953936179-d2a4734c5490?ixlib=rb-1.2.1&ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&auto=format&fit=crop&w=1400&q=80',
        // thumb:
        // 'https://images.unsplash.com/photo-1588953936179-d2a4734c5490?ixlib=rb-1.2.1&ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&auto=format&fit=crop&w=240&q=80',
        // subHtml: `<div class="lightGallery-captions">
        // <h4>Photo by <a href="https://unsplash.com/@jxnsartstudio">Garrett Jackson</a></h4>
        // <p>Published on May 8, 2020</p>
        // </div>`,
        // },
        // ],
    }),
    methods: {
        onInit: (detail: any) => {
            lightGallery = detail.instance;
        },
        updateSlides: function () {
            this.items = [
                ...this.items,
            ]
            lightGallery.refresh();
        },
        onBeforeSlide: () => {
            console.log('calling before slide');
        },
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
