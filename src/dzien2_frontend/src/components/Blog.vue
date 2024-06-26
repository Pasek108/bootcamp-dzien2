<template>
    <div>
        <div class="w-100 mb-4 flex flex-row justify-between">
            <h2 class="text-blue-600 inline-block">Wpisy na bloga</h2>
            <button @click="getPosts" class="border border-gray-700 rounded-sm px-2 cursor-pointer">Odśwież wpisy</button>
        </div>
        
        <div class="grid mx-6 gap-2 mb-4">
            <div v-for="(post, index) in posts" class="flex flex-row justify-between drop-shadow-sm bg-stone-300 px-4 py-1">
                <div>
                    <span class=" inline-block w-20">{{ index }}. </span>
                    <span>{{ post }}</span>
                </div>
                <div>
                    <button v-on:click="editPost(index)" class="border border-gray-700 rounded-sm px-2 bg-blue-500 cursor-pointer">Edit</button>
                    <button v-on:click="deletePost(index)" class="border border-gray-700 rounded-sm px-2 bg-red-500 cursor-pointer">Delete</button>
                </div>
            </div>
        </div>
        
        <input v-model="newPost" type="text" class="mb-4 border border-gray-700 rounded-sm px-2">
        <button @click="addPost" class="ml-2 mb-4 border border-gray-700 rounded-sm px-2 cursor-pointer">Dodaj wpis</button>
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
            await dzien2_backend.add_post(this.newPost).then(res => this.getPosts())
        },
        async editPost(id) {
            let new_text = prompt("Edit post:")
            await dzien2_backend.edit_post(id, new_text).then(res => this.getPosts())
        },
        async deletePost(id) {
            await dzien2_backend.delete_post(id).then(res => this.getPosts())
        },
        async getPosts() {
            this.posts = await dzien2_backend.read_posts()
        }
    },
    async mounted() {
        this.getPosts()
    }
}
</script>