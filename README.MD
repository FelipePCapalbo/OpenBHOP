First Person Reference: https://github.com/not-fl3/macroquad/blob/master/examples/first_person.rs

Particles Reference: https://github.com/not-fl3/particles-editor

Physics Reference: https://github.com/ValveSoftware/source-sdk-2013/blob/master/src/game/shared/gamemovement.cpp

SoundTrack Reference: 
```text
•°CYBORG SHARKS°•
- Track_1_birds.wav
- Track_2_amenbreak.wav
```

---

```text
src/
├── main.rs                 # Application entry point and lifecycle coordinator.
├── config.rs               # Global constants (physics variables and window settings).
├── input.rs                # InputService (abstracts mouse delta and keyboard axis).
├── audio.rs                # AudioService (loads and plays pitch-varied jump sounds).
├── game.rs                 # Global GameState manager (orchestrates update and draw passes).
├── world/                  # World map and environment definitions.
│   ├── mod.rs              # Module index and path exports.
│   └── environment.rs      # Environment geometry (renders grids and background).
├── hud/                    # Head-Up Display domain.
│   ├── mod.rs              # Module index and path exports.
│   └── hud.rs              # Renders on-screen statistics (telemetry speed and position).
└── player/                 # Encapsulated player domain.
    ├── mod.rs              # Module index and path exports.
    ├── player.rs           # Player entity orchestrator.
    ├── camera.rs           # First-person camera math (yaw, pitch, vectors).
    └── movement/           # Segregated physics and kinematics sub-domain.
        ├── mod.rs          # Module index and path exports.
        ├── kinematics.rs   # Movement orchestrator and spatial state syncer.
        ├── locomotion.rs   # XZ-plane translation and input mapping.
        ├── telemetry.rs    # Speed scalar calculation and telemetry metrics.
        └── jump/           # Segregated vertical physics and jump controls.
            ├── mod.rs      # Module index and path exports.
            ├── gravity.rs  # Vertical physics, gravity, and floor collisions.
            └── jump.rs     # Jump trigger logic.
```

---

## 3D Concepts 

To achieve an organic first-person experience without a commercial engine, we manually calculate the camera direction using **Euler Angles** and spatial vectors.

### Yaw and Pitch (The Head Rotation)

Instead of manipulating a complex 3D rotation matrix directly when moving the mouse, we split rotation into two simple angles:

* **`yaw`**: The horizontal rotation (turning your head left or right). It represents swinging around the Y-axis.
* **`pitch`**: The vertical rotation (looking up or down). It represents tilting around the X-axis. We lock (*clamp*) this value between `-1.5` and `1.5` rads to prevent the camera from flipping upside down.

---

### Camera Direction Vectors

Plates and graphic APIs require precise mathematical trajectories to position the player's view:

* **`camera_front`**: A 3D unit vector ($X, Y, Z$) pointing exactly where your eyes are looking. Every frame, we compute this vector using trigonometric functions over the updated `yaw` and `pitch` values:
* X = yaw.cos() * pitch.cos()
* Y = pitch.sin()
* Z = yaw.sin() * pitch.cos()

* **`camera_up`**: Defines the "sky" direction relative to the camera matrix, preventing the screen from tilting sideways. For standard walking mechanics, it stays locked pointing straight up `(0.0, 1.0, 0.0)`.

---

### Controls:

* `W`, `A`, `S`, `D` / `Arrow Keys`: Walk around on the XZ-plane.
* `Mouse`: Look around (Camera control).
* `Spacebar`: Jump (Simple linear gravity physics applied).
* `Tab`: Toggle cursor lock (unlocks mouse to close/move window).
* `Escape`: Instantly close the application.

---

### Psychoacoustic Audio (Shepard Tone)

To allow the jump audio pitch to scale indefinitely with player speed without hitting clipping thresholds or artificial limits, we implement a discrete **Shepard Tone** (Risset Glissando variation) at the moment of jumping:

* **Phase Loop:** The current horizontal speed is mapped to a cyclic phase between `[0.0, 1.0)`.
* **Harmonic Layers:** We simultaneously play two instances of the jump sound separated by exactly one octave (a *Sub* layer from `0.5x` to `1.0x` and a *Base* layer from `1.0x` to `2.0x`).
* **Equal-Power Crossfade:** Using trigonometric volume scaling ($\sin$ and $\cos$), the volume of the higher octave fades out while the lower octave fades in. This creates an acoustic illusion of a pitch that climbs infinitely alongside the speed.

---