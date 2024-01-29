import { setSelectedParticle, PARTICLE_PROPERTIES } from "./shared";
function createButton(particle) {
    const button = document.createElement("button");
    button.classList.add("particle-button");
    button.style.backgroundColor = `rgb(${particle.color.r}, ${particle.color.g}, ${particle.color.b})`;
    button.textContent = particle.name;
    button.addEventListener("click", () => {
        const cur = document.getElementById("selected-button");
        if (cur){
            cur.id = ""
            cur.classList.remove("selected")
        }
        button.id = "selected-button"
        button.classList.add("selected");
        console.log(`Selected particle type: ${particle.name}`);
    });
    setSelectedParticle(particle);

    return button;
}

export function setupController(element){
    
    document.querySelector('#controller').innerHTML = `
    <div id="controller" class="flex justify-center items-center gap-10 p-2 w-full max-w-2xl">
        <div class="text-2xl font-bold mb-10 heading">Particle Controller</div>
          <div id="buttons" class="w-full">
            <button id="play-button" class="bg-black hover:bg-slate-200">Play</button>
            <button id="pause-button" class="bg-black hover:bg-slate-2">Pause</button>
            <button id="reset-button" class="bg-black hover:bg-slate-2">Reset</button>
          </div>
        </div>
    </div>
    `
    document.getElementById("play-button").addEventListener("click", () => {
        console.log("Play");
    });
    
    document.getElementById("pause-button").addEventListener("click", () => {
        console.log("Pause");
    });
    
    document.getElementById("reset-button").addEventListener("click", () => {
        console.log("Reset");
    });
    
    const btns = document.getElementById("buttons")
    PARTICLE_PROPERTIES.forEach((particle) => {
        const button = createButton(particle);
        btns.appendChild(button);
    });
}