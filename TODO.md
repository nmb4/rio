# TODO

## [x] Fix sidebar tab text vertical alignment

Adjust the tab text alignment in the sidebar.

- The current tab text appears one or two pixels too low.
- Correct the vertical alignment so it is centered visually.

## [x] Fix the tab number indicator pill in the island

Adjust the tab number indicator pill shown in the island.

- The text inside the pill is vertically misaligned.
- The pill is too tall and should be narrower on the Y axis.
- The pill may need to be slightly wider.
- The text may need to be slightly narrower so it fits better within the reduced height.

## [x] Add a 500ms sidebar debounce for keyboard shortcuts

Add a delay before showing the sidebar when it is triggered through keyboard shortcuts.

- When the user switches tabs or triggers sidebar visibility via keyboard shortcuts, wait 500ms before showing the sidebar.
- If the interaction completes or changes again within that window, avoid flashing the sidebar immediately.
- The goal is to keep keyboard-driven navigation from causing distracting sidebar pop-ins.
- Preserve immediate or existing behavior for pointer-driven interactions unless the implementation shows they should share the same timing.

## Animate tab index changes

Animate the tab index indicator when switching between tabs.

- When switching from tab 2 to tab 3, animate the `2` upward and animate the `3` in from below.
- Make the behavior feel like a ticker.
- Use subtle motion to add polish without overwhelming the interface.

## Preserve the native macOS-style floating sidebar design

Keep the current visual direction of the sidebar, which feels like native macOS and works well as a floating panel.

- Preserve the minimal visual style.
- Treat the floating sidebar as the current default design direction.
- Avoid unnecessary visual complexity unless it supports a specific interaction.

## Add configurable Git information to sidebar items

Add a config-driven option to show Git information in the sidebar, similar to Zed.

- Add a `show_git_info`-style config option.
- Show the current branch on a second line.
- Show the amount of changes on the right side.
- Use a slightly smaller font for the Git info.
- Add a divider and a darker/subtler color for contrast.
- Keep the design minimal and avoid icons for this treatment.

## Add a bottom-right new-tab button

Consider adding a mouse-accessible plus button for new tabs.

- Place the plus button at the bottom right of the sidebar.
- Use it to create new tabs.
- Keep this separate from larger concepts like Arc-style spaces until those are better worked out.
- Do not prioritize spaces yet; it is still unclear whether they make sense or whether users should simply open new terminal windows.


# Future Work

## Add a config toggle for a connected slide-panel sidebar

Explore a config toggle that removes the left gap so the sidebar looks like a connected slide-in panel instead of a floating panel.

- When enabled, the sidebar should connect directly to the window border.
- Remove the gap between the sidebar and the left window edge.
- Remove rounded corners on the connected side.
- Keep the current floating version as an option.
- Validate whether the styling and layout make this feasible.

## Add pinned, side-positioned, and embedded sidebar modes

Explore additional sidebar layout modes beyond the current floating version.

- Add an option to pin the sidebar.
- Consider allowing the sidebar to be placed on the right side.
- Right-side placement may be useful because commands are usually on the left due to left-to-right text flow.
- Add an embedded mode where the sidebar is part of the window layout instead of floating.
- In embedded mode, shift the actual terminal panes to the right so the sidebar is baked into the layout.
- Work out the border styling for embedded mode.

## Add antialiasing support for line drawing

Improve 1px line rendering so embedded sidebar dividers and similar UI lines can use proper softened ends and edges.

- Investigate reusing the antialiasing approach already used for rounded rect corners.
- Prefer a renderer-level primitive rather than hand-assembling cap pixels in each UI component.
- Use this to replace the current manual embedded sidebar divider cap treatment.

## Detect interactive vs blocking commands in the sidebar

Add command-state detection for the Rio sidebar so it can distinguish between commands that should be waited on and commands that are interactive or long-running.

- Detect whether a command has entered an "all screen" or interactive state.
- If a command is interactive, do not wait for it to finish.
- For normal commands, continue waiting while they run, as with typical `cargo` commands.
- Treat known hot-reloading/dev commands such as `npm run dev` as non-blocking.
- Maintain a list of known commands and whether each one is blocking or interactive/non-blocking.
- Use this state to show a visual running indicator, such as an hourglass or loading spinner.

## Reintroduce bookmark colors

Bring back bookmark colors in the sidebar, similar to the earlier implementation before they were removed.

- Reintroduce colors in a way that fits the current minimal design.
- Consider applying the bookmark color to the tab pill border.
- Consider applying the bookmark color to the full tab pill background.
- If the full tab pill uses the bookmark color, invert the text color where needed.
- For colors like orange, black text may be needed.
- If text is inverted, it may need to be bold enough to maintain contrast.
- Be careful because a fully colored pill could become too intrusive or visually foreign to the current design.
(these two belong together, its the same feature, will clarify my intent when implementing)
## Explore a vertical color indicator before tab pills

Use a small vertical indicator before each tab pill to show bookmark color without making the full tab too visually heavy.

- Take inspiration from the Windows 11 taskbar behavior:
  - No indicator when an app has no windows.
  - A dot when it has windows but is not focused.
  - A wider pill when it is focused.
- Adapt this vertically for Rio tab pills.
- Place the indicator before the tab pill, with a small gap between the indicator and the pill.
- Consider a paired-rectangle design:
  - The left narrow rectangle uses rounded corners on its left side and sharp corners on its right side.
  - A small gap separates it from the tab pill.
  - The tab pill has sharp corners on its left side and rounded corners on its right side.
- Make the two shapes feel visually related, as if one rounded pill has been cut into two pieces.
- This may allow bookmark colors to work without feeling intrusive.

## Add update availability UI

Add an update button or indicator for Rio, inspired by Codex App or Arc-style update affordances.

- Add logic to check whether an update is available for Rio.
- Avoid placing a browser-like update button at the bottom of the sidebar if it feels too much like a browser.
- Prefer adding an update pill near the existing tab index indicator pill.
- The pill can simply say `Update`.
- Clicking it should start the update flow in the background.

## Prototype a safe self-update flow

Implement an initial placeholder self-update flow before solving the full cross-platform update mechanism.

- On update click, spawn a background process.
- The background process should first wait until Rio is killed.
- Attempt to terminate Rio in a SIGINT-like way.
- Add a fail-safe confirmation asking whether the user really wants to end all Rio processes, similar to Rio's existing close behavior.
- After Rio exits, run the update process.
- Start with a dummy implementation that does not actually update yet.
- After the dummy update process, respawn Rio.
- Defer the real cross-platform update implementation until later.

## Animate the update button expansion

Explore an update button animation inspired by a compact circle expanding into a labeled pill.

- In its resting state, the update button should look like a small circle that belongs near the traffic lights.
- Use an orange or blue circle with a white upward arrow for the update icon.
- On hover, animate the circle wider and reveal the `Update` text.
- Consider placing it near the top right.
- On Windows, it may need to be shifted due to platform window controls.
- Deprioritize Windows-specific polish for now; it can be handled later.

## Accept some platform-specific technical debt for now

Do not over-invest in Windows-specific details or a fully polished cross-platform update story yet.

- The current priority is the personal fork and core sidebar experience.
- It is acceptable to defer Windows-specific layout adjustments.
- It is acceptable to start with placeholder update behavior before implementing the full cross-platform version.
