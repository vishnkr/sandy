

export const PARTICLE_PROPERTIES = {
    "Sand": { name: "Sand", color: { r: 223, g: 175, b: 89 } },
    "Water": { name: "Water", color: { r: 52, g: 108, b: 202 } },
    "Stone": { name: "Stone", color: { r: 101, g: 106, b: 115 } },
    "Ice": { name: "Ice", color: { r: 195, g: 154, b: 247 } },
    "Fire": { name: "Fire", color: { r: 255, g: 123, b: 36 } },
    "Acid": { name: "Acid", color: { r: 15, g: 177, b: 15 } },
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