<template>
    <div style="align-items: center; align-self: center; text-align: center"
        class="d-flex align-items-center justify-content-center">
        <h2 style="padding-bottom: 2rem">gallery ig</h2>
        <!-- <h2>{{ selectedTags }}</h2> -->
        <div class="filter">
            <template v-for="(tag, i) in tags" :key="i">
                <input type="checkbox" :id="tag.id.toString()" @change="updateTags(tag.id)" checked />
                <label :for="tag.id.toString()">{{ tag.name }}</label>
            </template>
        </div>
        <lightgallery id="gallery" :settings="{ speed: 500, plugins: plugins }" :onInit="onInit"
            :onBeforeSlide="onBeforeSlide">
            <template v-for="(post, i) in posts" :key="i" :data-lg-size="200" className="gallery-item" :data-src="post" >
                <div v-if=tagCheck(post) v-html=renderPost(post)></div>
            </template>
        </lightgallery>
    </div>
</template>

<script lang="ts">
import axios from 'axios'
import lgVideo from 'lightgallery/plugins/video'
import lgZoom from 'lightgallery/plugins/zoom'
import { endianness } from 'os'
import { defineComponent, ref, type Ref } from 'vue'

type Response = {
    posts: Post[]
    tags: Tag[]
}
type Media = {
    src: String
    media_type: String
}
type Tag = {
    id: String
    name: String
}

type Post = {
    title: String
    author: String
    medias: Media[]
    tags: Tag[]
}

const DefaultTag: Tag = {
    id: "Something bad happened",
    name: "uhh"
}
let res: Response = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3002/v1"}/links`).then(
    res => res.data
)

let selectedTags: Ref<Tag[]> = ref(res.tags)

const tagCheck = (t: Post) => {
    let toShow = selectedTags.value.some((item, i) => {
        // console.log(`${item.id} ${t.tags[i].id}`)
        return t.tags.map(x => x.id).indexOf(item.id) !== -1;
    })
    // console.log(`${t.src} is ${toShow} as it has ${t.tags.map(x => x.name).join(' and ')}`)
    return toShow
}
const renderPost = (post: Post) => {
    let returnCase : String = `<div class="post"><p style="align-self: left;">${post.title}</p>`;
    for (let media of post.medias) {
        returnCase += (media.media_type.startsWith('video') ? `<video style="align-self: center; width: 100%" controls>
                    <source src="${media.src}" type="video/mp4" />
                </video>`  :      `<img className="img-responsive" src="${media.src}"
                    style="align-self: center; width: 100%" />`)
    }
    returnCase += `<p style="align-self: right;">${post.author}</p>\n</div>`;
    return returnCase
;
}

export default defineComponent({
    name: 'App',
    components: {
        // Lightgallery,
    },
    data: () => ({
        plugins: [lgZoom, lgVideo],
        posts: res.posts,

        // items:  res.lin,
        tags: res.tags,
        selectedTags: selectedTags
    }),

    methods: {
        onInit: () => {
            console.log('initalised')
        },
        onBeforeSlide: () => { },
        updateTags: (x: String) => {
            if (
                selectedTags.value
                    .map((e) => {
                        return e.id
                    })
                    .includes(x)
            ) {
                selectedTags.value = selectedTags.value.filter((f) => f.id != x)
            } else {
                selectedTags.value.push(res.tags.find((o) => o.id == x) ?? DefaultTag)
            }

        },
        renderPost: (post: Post) => { return renderPost(post) },
        tagCheck: (t: Post) => { return tagCheck(t) }
    }
})
</script>
<style lang="css">
@import url('https://cdn.jsdelivr.net/npm/lightgallery@2.1.0-beta.1/css/lightgallery.css');
@import url('https://cdn.jsdelivr.net/npm/lightgallery@2.1.0-beta.1/css/lg-zoom.css');

body {
    margin: 0;
}

.gallery-item {
    margin: 5px;
}
</style>
