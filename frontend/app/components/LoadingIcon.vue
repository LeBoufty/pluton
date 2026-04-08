<template>
    <div>
        <Centerer>
            <img class="sun" src="~/assets/img/loading/sun.png" />
        </Centerer>
        <div class="loading_leftright">
            <img class="loading_updown" src="~/assets/img/loading/planet.png" />
        </div>
    </div>
</template>

<style>
.sun {
    position: absolute;
    margin-top: 8em;
    margin-left: 14em;
    width: 7%;
}
.loading_leftright {
    position: absolute;
    width: 100vw;
    height: 100vh;
    animation: leftRight 2s alternate infinite ease-in-out;
}
.loading_updown {
    position: absolute;
    animation: upDown 2s alternate infinite ease-in-out;
    animation-delay: 1000ms;
    width: 3%;
}

@keyframes upDown {
    to {
        transform: translatey(100px);
        opacity: if(media(y < 0): 0 else 1);
    }
}
@keyframes leftRight {
    to {
        transform: translatex(200px);
    }
}
</style>
