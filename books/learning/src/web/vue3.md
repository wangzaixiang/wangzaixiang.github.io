# understand vue3

```html

import {defineProps, ref} from 'vue';
export default {
    props: ["title"], // same as defineProps(["title"]) in setup
    setup() {   // return object is merged with this
        const count = ref(0);
        defineProps(["title"]); // a compile-time macros
        return {
            count
        }
    }
    template: `<button @click="count++">Click me {{count}}</button>`,

    render() {  // 会根据 template 编译这个函数
    }
    __file: "HelloWorld.vue",


}

```

`<script setup>` 会编译为一个 setup 函数，script 中的 top level 会成为 this 的可访问变量。
`<script>` export 一个 obj, 会 merge 到 this 中。

SFC 是 上述的一个语法糖
