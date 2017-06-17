A lighting controller built on two primitive ideas:
- analog clocks:
    - "tells the time" in a continuous fashion (eg it is presently 35.75935% of the way between the last tick and the next tick)
    - "ticks" (supplies a trigger event when the clock's phase rolls from 1.0 to 0.0)

- quasi-periodic waveforms: waveforms with a current phase on [0.0,1.0)
    You may ask a waveform for its current value, or its current value with an arbitrary phase offset.
    Waveforms need not be truly periodic but should still have a notion of constant-width periods in time (representing the portion of the waveform mapped to the phase 0.0 to 1.0) with phase offsets (possibly returning a value from a neighboring period, regardless of whether the waveform is 1.0-periodic)