# Learning OpenGL

In this repo I'll be documenting my progress in learnign OpenGL in rust language.
My main information source is the book [Learn OpenGL - Graphics Programming](https://learnopengl.com/)
which is originally written for C++.


This book uses the GLFW library for OpenGL desktop development, instead, I'll be using
SDL2, since its rust port is much more popular.


## Branches

Each branch contains a different stage at my learning. Each branch name starts
with the chapter in the book which I followed before writing the code.


The main branch is simply the first chapter - "Hello window"
in which I only create a window and draw something basic inside it.

## Section I - Getting started

In this chapter we are getting familiar with all the basics that make all 
this ecosystem run. We are integrating `GLFW` (I'm using `SDL2` instead) with `OpenGL`
(I'm using `gl-rs`)


By the end of the section I will be drawing triangles to the screen (yes, that's it for now).