<template>
    <div>
        <button @click="getPosts">Odśwież wpisy</button>
        Siema blog
        {{ posts }}
        <input v-model="newPost" type="text">
        <button @click="addPost">Dodaj wpis</button>
    </div>
</template>

<script>
import { dzien2_backend } from 'declarations/dzien2_backend/index';

export default {
    data() {
        return {
            posts: [],
            newPost: ""
        }
    }, 
    methods: {
        async addPost () {
            await dzien2_backend.add_post(this.newPost)
        },
        async getPosts() {
            this.posts = await dzien2_backend.read_posts()
        }
    },
}
</script>