<template>
    <Transition>
        <div class="loading_wrapper">
            <div class="system">
                <div class="sun">
                    <img src="~/assets/img/loading/sun.png" />
                </div>
                <div class="planet">
                    <img src="~/assets/img/loading/planet.png" />
                </div>
            </div>
        </div>
    </Transition>
</template>

<style>
/* repeatedly hit my head with a hammer trying to do this
until this goat saved my life (and my skull)
https://codersblock.com/blog/making-orbit-animations-with-css-custom-properties/ */

@property --angle {
    syntax: "<angle>";
    inherits: true;
    initial-value: 0deg;
}

@property --z {
    syntax: "<integer>";
    inherits: true;
    initial-value: 0;
}

@keyframes revolve {
    from {
        --angle: -180deg;
        --z: 0;
    }
    to {
        --angle: 180deg;
        --z: 100;
    }
}

.loading_wrapper {
    overflow: hidden;
}

.system {
    display: grid;
    place-items: center;
    height: 100vh;
    margin: 0;
    padding-top: 17vh;
    background-color: #141733;
    overflow: clip;
}

.planet {
    --x-amplitude: 100px;
    --y-amplitude: 40px;
    --x: calc(cos(var(--angle)) * var(--x-amplitude));
    --y: calc(sin(var(--angle)) * var(--y-amplitude) - 50vh);
    translate: var(--x) var(--y);
    z-index: calc(var(--z)); /* calc() is for Safari */
    animation: revolve 6s linear infinite;
}

img {
    display: block;
}

.sun {
    z-index: 50;
}

.sun img {
    width: 100%;
}

.planet img {
    width: 100%;
}
</style>
