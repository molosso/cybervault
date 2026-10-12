<script setup>

import {ref,onMounted} from "vue"

import {invoke} from "@tauri-apps/api/core"



const items=ref([])


const name=ref("")
const login=ref("")
const secret=ref("")

const search=ref("")



async function load(){

items.value=
await invoke("list_items")

}



async function add(){


await invoke(

"add_item",

{

name:name.value,

login:login.value,

secret:secret.value

}

)


name.value=""

login.value=""

secret.value=""


load()

}



async function remove(id){

await invoke(

"remove_item",

{id}

)

load()

}



async function generate(){


secret.value=

await invoke("generate")


}



onMounted(load)



</script>



<template>


<div class="castle">


<h1>
⚔ CYBER VAULT ⚔
</h1>


<p class="subtitle">

The Forgotten Digital Grimoire

</p>



<div class="panel">


<input

v-model="name"

placeholder="Realm / Website"

/>


<input

v-model="login"

placeholder="Username"

/>



<div class="row">


<input

v-model="secret"

placeholder="Secret"

/>


<button @click="generate">

Forge

</button>


</div>



<button @click="add">

Seal Entry

</button>


</div>



<input

class="search"

v-model="search"

placeholder="Search the vault"

/>



<div

v-for="item in items.filter(x=>x.name.toLowerCase().includes(search.toLowerCase()))"

:key="item.id"

class="scroll"


>


<h2>

{{item.name}}

</h2>


<p>

☩ {{item.login}}

</p>


<p>

☠ {{item.secret}}

</p>


<button

@click="remove(item.id)"

>

Burn Scroll

</button>


</div>



</div>


</template>