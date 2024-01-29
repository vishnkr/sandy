const sharedState = {
    selectedParticle: "Sand",
};

export const PARTICLE_PROPERTIES = [
    { color: { r: 223, g: 175, b: 89 }, name: "Sand" },
    { color: { r: 52, g: 108, b: 202 }, name: "Water" },
    { color: { r: 101, g: 106, b: 115 }, name: "Stone" },
    { color: { r: 195, g: 154, b: 247 }, name: "Ice" },
    { color: { r: 255, g: 123, b: 36 }, name: "Fire" },
    { color: { r: 15, g: 177, b: 15 }, name: "Acid" },
];

export function setSelectedParticle(particle) {
    sharedState.selectedParticle = particle;
}

export function getSelectedParticleProps() {
    const selectedParticle = sharedState.selectedParticle;
    
    return PARTICLE_PROPERTIES.find(particle => particle.name === selectedParticle.name);
}