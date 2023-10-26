<template>
    <!-- <button v-on:click="updateSlides">Add new image</button> -->
    <div id="gallery" data-nanogallery2></div>
</template>

<script lang="ts">
import axios from 'axios';
import { stringifyQuery } from 'vue-router';
import { defineComponent } from 'vue';
import $ from 'jquery';
import 'nanogallery2';


console.log(import.meta.env.VITE_API_URL)
// const links = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3000/v1"}/links`).then(
    // res => res.data
// ).then(
    // res => {
        // return res as String[]
    // }
// )
export default defineComponent({
    // name: 'App',
    // components: {
    // },
    // data: () => ({
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
    // })
});
$(document).ready(function () {
    $("#gallery").nanogallery2({
      items: [
        {
          src:          'https://vimeo.com/32875422',                           // media url
          srct:         'https://i.vimeocdn.com/video/222726041_1280x720.jpg',  // media thumbnail url
          title:        'Vimeo video',                                          // media title
          description:  'Video'                                                 // media description
        },
        { src: 'https://www.youtube.com/watch?v=Ir098VWCv8Q', title: 'Youtube video' },
        { src: 'http://www.dailymotion.com/video/x4wtyl6',    title: 'Dailymotion video' },
        { src: 'image_01.jpg',      srct: 'image_01ts.jpg',   title: 'Title Image' }
      ]
    });
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
