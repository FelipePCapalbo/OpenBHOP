First Person Reference: https://github.com/not-fl3/macroquad/blob/master/examples/first_person.rs

Particles Reference: https://github.com/not-fl3/particles-editor

Physics Reference: https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/shared/gamemovement.cpp

SoundTrack Reference: 
```text
•°CYBORG SHARKS°•
- Track_1_birds.wav
- Track_2_amenbreak.wav
- Track_3_silvercap.wav
```

---

```text
src/
├── main.rs                         # Application entry point and lifecycle coordinator.
├── config.rs                       # Global constants (physics variables and window settings).
├── input.rs                        # InputService (abstracts mouse delta and keyboard axis).
├── game.rs                         # Global GameState manager (orchestrates update and draw passes).
├── audio/                          # Audio subsystem and audio services.
│   ├── mod.rs                      # Module index and path exports.
│   ├── audio.rs                    # Audio service definitions/interfaces.
│   ├── jump_audio.rs               # Handles playing and scheduling jump sounds.
│   ├── playlist.rs                 # Manages music playback and soundtrack.
│   └── metronome/                  # Metronome system for rhythmic jumps.
│       ├── mod.rs                  # Module index.
│       ├── metronome.rs            # Core metronome logic.
│       ├── metronome_dynamics.rs   # Controls dynamic behaviour based on speed/combo.
│       └── metronome_sound.rs      # Plays the metronome ticks.
├── world/                          # World map, environment, and terrain definitions.
│   ├── mod.rs                      # Module index and path exports.
│   ├── environment.rs              # Environment geometry (renders grids and background).
│   ├── floor/                      # Dynamic floor/terrain generation.
│   │   ├── mod.rs                  # Module index.
│   │   └── floor_generation.rs     # Generates platforms and track floor layout.
│   └── gorgonzoi/                  # Gorgonzoi object/world mechanics.
│       ├── mod.rs                  # Module index.
│       ├── gorgonzoi.rs            # Renders the gorgonzoi entities/structures.
│       └── gorgonzoi_mesh.rs       # 3D mesh building helper for Gorgonzoi.
├── hud/                            # Head-Up Display domain.
│   ├── mod.rs                      # Module index and path exports.
│   ├── hud.rs                      # Renders main on-screen statistics (telemetry speed and position).
│   ├── effects/                    # Special HUD effects and camera/display sway.
│   │   ├── mod.rs                  # Module index.
│   │   ├── effects.rs              # General effects pipeline.
│   │   ├── bubble.rs               # Speed bubble or air particles effect.
│   │   └── sway.rs                 # Camera sway/HUD sway logic.
│   ├── minimap/                    # Radar/minimap UI display.
│   │   ├── mod.rs                  # Module index.
│   │   └── minimap.rs              # Computes and draws player position on map.
│   └── gorgonzoi/                  # Gorgonzoi HUD visuals, combos and particles.
│       ├── mod.rs                  # Module index.
│       ├── bhop_combo.rs           # BHOP combo system UI and metrics.
│       ├── gorgonzoi_hud.rs        # Renders Gorgonzoi indicators.
│       ├── gorgonzoi_mesh.rs       # Pre-calculated or mesh-based HUD graphics.
│       └── particles_stage.rs      # Particle effects for BHOP combo level-up/stages.
└── player/                         # Encapsulated player domain.
    ├── mod.rs                      # Module index and path exports.
    ├── player.rs                   # Player entity orchestrator.
    ├── camera.rs                   # First-person camera math (yaw, pitch, vectors).
    └── movement/                   # Segregated physics and kinematics sub-domain.
        ├── mod.rs                  # Module index and path exports.
        ├── kinematics.rs           # Movement orchestrator and spatial state syncer.
        ├── locomotion.rs           # XZ-plane translation and input mapping.
        ├── telemetry.rs            # Speed scalar calculation and telemetry metrics.
        ├── vertical_physics.rs     # Vertical physics, gravity, and floor collisions.
        └── jump/                   # Segregated vertical physics and jump controls.
            ├── mod.rs              # Module index and path exports.
            └── jump.rs             # Jump trigger logic.
```

---

## Controls

* `W`, `A`, `S`, `D` / `Arrow Keys`: Walk around on the XZ-plane.
* `Mouse`: Look around (Camera control).
* `Spacebar`: Jump.
* `Tab`: Toggle cursor lock (unlocks mouse to close/move window).
* `Escape`: Instantly close the application.

---

## Game Mechanics & Physics

### Bunnyhopping & Air Acceleration
Bunnyhopping (BHOP) leverages a mathematical artifact of the **Source Engine (GoldSrc/Quake)** physics simulation. 
When the player is in the air, acceleration along the desired wish direction vector $\vec{w}$ is calculated as:
$$\Delta v = \min(a \cdot v_{wish} \cdot \Delta t, v_{wish\_cap} - \vec{v} \cdot \vec{w})$$
Where:
- $v_{wish}$ is the target wish speed.
- $v_{wish\_cap}$ is a restricted wish speed cap in the air.
- $\vec{v}$ is the current velocity vector.
- $\vec{w}$ is the normalized wish direction vector.
- $a$ is the air acceleration coefficient.

If the projection of the current velocity onto the wish direction ($\vec{v} \cdot \vec{w}$) is less than $v_{wish\_cap}$, the player gains speed. By rotating the camera yaw and holding strafe keys (keeping $\vec{v} \cdot \vec{w}$ near zero or negative), the player can infinitely accumulate horizontal speed without encountering friction or triggering the speed clamp.

### Gravity-Driven Metronome
To assist the player in timing their bunnyhops, a dynamic metronome plays downbeats and subdivisions aligned with the player's flight cycle. The metronome's timing is physically derived from kinematic equations of a jump under gravity:
$$T_{flight} = \frac{2 \cdot v_0}{|g|}$$
Where:
- $v_0$ is the jump launch velocity (`JUMP_FORCE`).
- $g$ is the gravitational acceleration (`GRAVITY`).

The metronome schedules audio events during this exact flight duration $T_{flight}$, firing downbeats at launch ($t = 0$) and landing ($t = T_{flight}$).

---

## World & Rendering

### 3D Camera Concepts
To achieve an organic first-person experience without a commercial engine, we manually calculate the camera direction using **Euler Angles** and spatial vectors.

#### Yaw and Pitch (The Head Rotation)
Instead of manipulating a complex 3D rotation matrix directly when moving the mouse, we split rotation into two simple angles:
* **`yaw`**: The horizontal rotation (turning your head left or right). It represents swinging around the Y-axis.
* **`pitch`**: The vertical rotation (looking up or down). It represents tilting around the X-axis. We lock (*clamp*) this value between `-1.5` and `1.5` rads to prevent the camera from flipping upside down.

#### Camera Direction Vectors
Plates and graphic APIs require precise mathematical trajectories to position the player's view:
* **`camera_front`**: A 3D unit vector ($X, Y, Z$) pointing exactly where your eyes are looking. Every frame, we compute this vector using trigonometric functions over the updated `yaw` and `pitch` values:
  - $X = \cos(\text{yaw}) \cdot \cos(\text{pitch})$
  - $Y = \sin(\text{pitch})$
  - $Z = \sin(\text{yaw}) \cdot \cos(\text{pitch})$
* **`camera_up`**: Defines the "sky" direction relative to the camera matrix, preventing the screen from tilting sideways. For standard walking mechanics, it stays locked pointing straight up `(0.0, 1.0, 0.0)`.

### Chunk-Based Terrain Generation
The terrain is rendered dynamically using a grid chunking system:
- The infinite world grid is divided into chunks of size $C \times C$ cells.
- Only chunks within a circular visual radius $R$ of the player are active in memory:
  $$(dx)^2 + (dz)^2 \leq R^2$$
- Field-of-View (FOV) culling uses the dot product to discard cells behind the player:
  $$\vec{d}_{cell} \cdot \vec{d}_{view} \geq 0.15$$
- Player path tracking is persisted in a binary file `visited_cells.bin` to mark cells as darker gray upon visitation.

### Gorgonzoi Rendering
The 3D Gorgonzoi are rendered procedurally:
- A triangular wedge mesh is generated dynamically via vertices and indices.
- A deterministic sine/cosine noise function evaluates coordinates to assign procedural mold textures:
  $$\text{noise} = \sin(12.5x)\cos(14.2y) + \sin(17.3z)\cos(8.9x)$$
  Values above a threshold render as greenish-blue mold, while others render as base cheese yellow.
- Aesthetic attributes (rotation speed, scale, flame particles) interpolate linearly based on the jump combo count.

---

## Psychoacoustic Audio (Shepard Tone)

To allow the jump audio pitch to scale indefinitely with player speed without hitting clipping thresholds or artificial limits, we implement a discrete **Shepard Tone** (Risset Glissando variation) at the moment of jumping:

* **Phase Loop:** The current horizontal speed is mapped to a cyclic phase between `[0.0, 1.0)`.
* **Harmonic Layers:** We simultaneously play two instances of the jump sound separated by exactly one octave (a *Sub* layer from `0.5x` to `1.0x` and a *Base* layer from `1.0x` to `2.0x`).
* **Equal-Power Crossfade:** Using trigonometric volume scaling ($\sin$ and $\cos$), the volume of the higher octave fades out while the lower octave fades in. This creates an acoustic illusion of a pitch that climbs infinitely alongside the speed.

---