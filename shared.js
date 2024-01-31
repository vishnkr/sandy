

export const PARTICLE_PROPERTIES = {
    "Sand": { name: "Sand", colors: [
        'rgb(223, 175, 89)',
        'rgb(221, 172, 75)',
        'rgb(234, 180, 80)',
        'rgb(239, 188, 93)'
    ] },
    "Water": { name: "Water", colors: [
        'rgb(52, 108, 202)',
        'rgb(85, 136, 213)',
        'rgb(83, 134, 209)',
    ]},
    "Stone": { name: "Stone", colors: ['rgb(101,106,115)'] },
    "Ice": { name: "Ice", colors: ['rgb(195,154,247)']},
    "Fire": { name: "Fire", colors: ['rgb(255,123,36)'] },
    "Acid": { name: "Acid", colors: ['rgb(15,177,15)']},
};

const sharedState = {
    selectedParticle:  { color: { r: 223, g: 175, b: 89 }, name: "Sand" },
};

export function setSelectedParticle(particle) {
    sharedState.selectedParticle = particle;
}

export function getSelectedParticleProps() {
    const selectedParticle = sharedState.selectedParticle;
    return PARTICLE_PROPERTIES[selectedParticle.name]
}