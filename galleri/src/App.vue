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
            <a v-for="(item, i) in items" :key="i" :data-lg-size="200" className="gallery-item" :data-src="item.src">
                <template v-if="tagCheck(item)">
                    <img v-if="!item.media_type.startsWith('video')" className="img-responsive" :src="item.src.toString()"
                        style="align-self: center; width: 100%" />
                    <video v-else className="" style="align-self: center; width: 100%" controls>
                        <source :src="item.src.toString()" type="video/mp4" />
                    </video>
                </template>

            </a>
        </lightgallery>
    </div>
</template>

<script lang="ts">
import axios from 'axios'
import lgVideo from 'lightgallery/plugins/video'
import lgZoom from 'lightgallery/plugins/zoom'
import { defineComponent, ref, type Ref } from 'vue'


console.log(import.meta.env.VITE_API_URL)

type Response = {
    items: Media[]
    tags: Tag[]
}
type Media = {
    src: String
    tags: Tag[]
    media_type: String
}
type Tag = {
    id: String
    name: String
}

const DefaultTag: Tag = {
    id: "Something bad happened",
    name: "uhh"
}
let res: Response = await axios.get(`${import.meta.env.VITE_API_URL ?? "http://localhost:3002/v1"}/links`).then(
    res => res.data
)
// )
// let res: Response = {, UNIX_EPOCH
// items: [
// {
// src: 'https://cdn.discordapp.com/attachments/1140180380683612160/1140564617077203047/huge_2023-08-14_16.26.39.png?ex=6551e013&is=653f6b13&hm=1ef2518ffe1ffd1742caa85dd22c800605a721d62ca0ab9c38283e4a1f615a86&',
// media_type: "video/mp4",
// tags: [
// {
// id: 'val',
// name: 'val'
// }, {
// id: 'osu',
// name: 'osu'
// }
// ]
// },
// {
// src: 'https://cdn.discordapp.com/attachments/1140180380683612160/1140564620046766100/huge_2023-08-14_16.24.44.png?ex=6551e014&is=653f6b14&hm=76450bbb2660afa4306198523e6b345069e990db653455f2f55b9fc52aa18432&',
// media_type: "video/mp4",
// tags: [{ id: 'osu', name: 'osu' }]
// }
// ],
// tags: [
// {
// id: 'osu',
// name: 'osu'
// },
// {
// id: 'val',
// name: 'val'
// }
// ]
// };

let selectedTags: Ref<Tag[]> = ref(res.tags)

const tagCheck = (t: Media) => {
    let toShow = selectedTags.value.some((item, i) => {
        // console.log(`${item.id} ${t.tags[i].id}`)
        return t.tags.map(x => x.id).indexOf(item.id) !== -1;
    })
    // console.log(`${t.src} is ${toShow} as it has ${t.tags.map(x => x.name).join(' and ')}`)
    return toShow
}

export default defineComponent({
    name: 'App',
    components: {
        // Lightgallery,
    },
    data: () => ({
        plugins: [lgZoom, lgVideo],
        items: res.items,

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
        tagCheck: (t: Media) => { return tagCheck(t) }
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
