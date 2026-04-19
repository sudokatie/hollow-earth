# Hollow Earth

You live on the inside of a sphere. The ground curves up. The sky is stone. Gravity pulls you outward, toward the shell. A burning core hangs in the distance, casting shadows that point the wrong way. Welcome home.

## Why This Exists?

Every survival game puts you on the outside of a ball. Boring. What if you're on the inside? What does gravity do when "down" is relative to where you stand? How do you navigate when walking straight takes you in a circle? These aren't just gimmicks -- they're fundamental changes to how movement, combat, and exploration work.

Hollow Earth is the third game built on the Lattice engine, and it stress-tests something different: non-euclidean space. The physics of living inside a sphere create emergent gameplay that flat worlds literally can't replicate.

## Features

- Radial gravity -- "down" is always away from the core, wherever you stand
- A hollow sphere 8192 blocks across with 64 blocks of mineable shell
- Non-euclidean navigation -- walk far enough and you come back around
- A luminous core that pulses day and night, 512 blocks of pure danger
- Radiation zones that scale with proximity to the core
- Freefall across the interior -- miss your footing, meet the opposite side
- 6 surface biomes from fungal forests to magma fields
- Tether system so you don't die falling through the void
- Gravity boots, core shields, and a containment suit for the brave
- Crystal pickaxes that refract light, spore grenades, gravity hammers

## Quick Start

```bash
git clone https://github.com/sudokatie/hollow-earth.git
cd hollow-earth
cargo run --release
```

Requires Rust 1.75+ and a GPU with Vulkan/Metal/DX12 support.

## Philosophy

1. Physics first. Gravity isn't a gimmick -- it's the entire design.
2. The sphere is the level. No loading screens, no boundaries.
3. Verticality matters. Cliffs, chasms, and freefall are core mechanics.
4. The core is the endgame. You can see it from spawn. Getting there is the journey.

## License

MIT

---

*If you fall off the inside of a sphere, where do you land? The other side.*
