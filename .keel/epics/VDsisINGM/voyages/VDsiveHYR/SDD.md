# Interactive Terminal Loop and Globe Control - Software Design Description

## Architecture

We will use a specialized TTY loop within the `atext globe` CLI path.

### TTY Management
- Put the terminal into **raw mode** to capture keys without buffering.
- Use **non-blocking reads** or a small timeout to keep the rendering loop fluid.
- Ensure terminal state is restored on exit.

### Rendering Strategy
- Calculate the globe view based on `angle_x` and `angle_y` state variables.
- Print the frame.
- Instead of newline, use `\x1b[H` (Home) or `\x1b[A` (Cursor Up) to return to the start of the globe area.
- This creates a single-buffer flicker-free animation in most modern terminals.

### Input Mapping
- `Up/Down`: Adjust `angle_x`.
- `Left/Right`: Adjust `angle_y`.
- `Q`: Break loop and exit.
