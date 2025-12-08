# What is Metor?
Metor is an experimental systems language I'm developing during my free time at MIT. Metor's goal is to achieve the performance of a system language but actually be enjoyable to use. Metor's design is informed by my favorite parts from C#, TypeScript, and Rust, while also doing away with the things I despise about C++ and Rust.

# What does Metor Look Like?
> [!WARNING]  
> Metor is currently under rapid and active developement and all syntax is highly subject to change.
Here's my initial view for how a simple program my look like in Metor:
(This is an example solution for this problem: https://leetcode.com/problems/h-index/description/)
```ts
import { max } from Math
import { HashMap } from Standard

i32 hIndex: i32[] citations {
    const HashMap<i32, i32> dict = HashMap.new()
    
    for citation in citations {
        if !dict.hasKey(citation) dict.set(citation, 0)

        dict.set(citation, dict.get(citation) + 1)
    }

    i32 maxH = 0

    for key in dict.keys() {
        if dict.get(key) >= key {
	        maxH = max(maxH, key)
        }
    }

    return maxH
}
```

# What does Metor do Different?
I was driven to create Metor mainly from two gribes. I despise how messy the build system and package management ecosystem is for C++. I also have grown to dislike how begrugingly slow it feels to write Rust. Now before all of the Rust fanboys come hunt me down and murder me in my sleep, just hear me out. While I am not an expert at rust, I have written a few projects in it. Namely my scriptable vulkan renderer and now this initial compiler (Eventually the Metor compiler will be written in Metor). I have always felt that I was working against the language to achieve what I wanted. I'm all for memory rigourous safety, but sometimes I just want to write the damn function.

# Compiler Design
The design of Metor's compiler is heavily informed by my experiements writing the [Mew compiler](https://github.com/outercloudstudio/Mew-Compiler/tree/main). I haven't done any formal reasearch into compiler design yet, so pretty much everything here is just me making stuff up on the fly. Although, I'll probably take a compilers class soon in the next few years.

## Why is it Called Metor?
Metor is a nickname some of my friends from highschool gave me and I think it's funny. :)