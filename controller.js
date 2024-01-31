import { setSelectedParticle, PARTICLE_PROPERTIES } from "./shared";
function createButton(particle) {
    const button = document.createElement("button");
    button.classList.add("particle-button");
    button.style.backgroundColor = particle.colors[0];
    button.textContent = particle.name;
    button.addEventListener("click", () => {
        const cur = document.getElementById("selected-button");
        if (cur){
            cur.id = ""
            cur.classList.remove("selected")
        }
        button.id = "selected-button"
        button.classList.add("selected");
        setSelectedParticle(particle);
    });

    return button;
}

export function setupController(element){
    
    document.querySelector('#controller').innerHTML = `
    <div id="controller" class="flex flex-wrap justify-center items-center gap-10 p-2 w-full max-w-2xl">
        <p>Frames per Second:</p>
        <span id="latest"></span>
        <span id="avg"></span>
        <div class="text-2xl font-bold mb-10 heading">Particle Controller</div>
          <div id="buttons" class="w-full flex">
            <button id="play-button" class="bg-black hover:bg-slate-200 m-2">Play</button>
            <button id="pause-button" class="bg-black hover:bg-slate-2 m-2">Pause</button>
            <button id="reset-button" class="bg-black hover:bg-slate-2 m-2">Reset</button>
          </div>
        </div>

    </div>
    `
    const btns = document.getElementById("buttons")
    Object.values(PARTICLE_PROPERTIES).forEach((particle) => {
        const button = createButton(particle);
        btns.appendChild(button);
    });
}