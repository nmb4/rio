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

## Make sidebar styling configurable via themes

Move sidebar visual styling into theme configuration.

- Expose floating and embedded sidebar colors through the theme.
- Include divider colors, active tab backgrounds, borders, text colors, opacity, and subtle edge treatment.
- Avoid hard-coded sidebar styling values in renderer code where a theme value would be more appropriate.

## [x] Fix Cmd+Backspace on macOS

Make `Cmd+Backspace` delete to the beginning of the current line on macOS.

- Implemented as a default macOS `Super+Backspace` binding that sends `Ctrl+U` (`\x15`).
- Covered by a focused binding test.
- Normal Backspace, Option+Backspace, search mode, and vi mode behavior are kept separate.

## Fix split-pane layout on initial open

Make split-pane layouts render correctly immediately after opening.

- The current layout may not update until the window is resized.
- Find the missing initial layout or resize update call.
- Ensure the fix covers single-pane tabs and split-pane tabs without requiring a manual resize.

## Fix floating sidebar tab list scrolling

Keep all tabs accessible when the floating sidebar has more tabs than fit vertically.

- Add overflow handling for the sidebar tab list.
- Support scrolling past the currently visible rows.
- Ensure active-tab selection remains visible when navigating by keyboard.
- Keep the scrollbar or scroll affordance subtle enough to match the current sidebar design.

## Fix close confirmation keyboard behavior on macOS

Make the close confirmation flow confirmable from the keyboard.

- The native macOS dialog cannot currently be confirmed with Enter.
- Prefer an in-app popup if the native dialog cannot be made keyboard-friendly.
- Reuse the Command Palette interaction style where practical.
- Support Enter to confirm, with `Cmd+Enter` as a possible safer alternative.

## Fix oversized parentheses and brackets rendering

Investigate and fix bracket glyph rendering where parentheses or brackets appear too large.

- Reproduce with Departure Mono and Berkeley Mono.
- Treat this as likely Rio/Sugarloaf shaping or font metric behavior rather than a font-specific issue.
- Check whether fallback font selection is involved before changing glyph metrics globally.

## Apply Set visual polish changes

Bring Set's matching UI constants in line with the Rio sidebar polish.

- Change the relevant border width from 2px to 1px for DPI behavior.
- Reduce opacity from 40% to the matching 20% or 10% treatment.
- Keep this tracked separately from Rio-specific sidebar rendering work.


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
- Explore right-aligning the floating sidebar so it does not overlap left-side command input.
- Add an embedded mode where the sidebar is part of the window layout instead of floating.
- In embedded mode, shift the actual terminal panes to the right so the sidebar is baked into the layout.
- Work out the border styling for embedded mode.

## Add compact and detail sidebar tab modes

Add a mode toggle for how much information each sidebar tab shows.

- Compact mode should be a single-line row with directory and command only.
- Detail mode should add Git info such as branch, staged changes, unstaged changes, push, and pull state.
- Consider adding run duration in Detail mode.
- Add subtle separators between tabs in Detail mode so multi-line tabs remain visually distinct.
- Keep the toggle configurable and compatible with the separate `show_git_info` option.

## Add modifier-revealed tab number badges

Show numbered badges on sidebar tabs while holding `Cmd`.

- Use a Raycast or Codex-style reveal interaction.
- Avoid requiring manual counting when switching to tabs by number.
- Keep the badges hidden during normal pointer use.

## Add configurable per-tab status icons

Show small contextual icons for important tab state.

- Show a sparkle icon when a coding agent is running.
- Show a pane icon when tmux or Zellij is active.
- Make the state-to-icon mapping configurable in the config file.
- Keep icon use optional so the minimal sidebar treatment remains available.

## Add Git information to the title bar

Show concise Git context in the title bar when sidebar mode is enabled.

- Use a format like `directory | program | git branch | git changes (+/-)`.
- Keep the information compact enough to coexist with the traffic lights and tab index indicator.
- Share Git-status plumbing with sidebar Git info where possible.

## Resolve the title-bar tab index indicator

Decide how the current tab index indicator should behave near the traffic lights.

- Decide whether to keep, move, or restyle the indicator.
- Coordinate this with the update affordance and tab-index animation work.
- Preserve enough visibility for quick tab orientation.

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

## Add notifications for completed long-running commands

Notify when long-running commands finish.

- Use system notifications similar to Warp's behavior.
- Tie notification eligibility to command-state detection.
- Avoid notifying for interactive or intentionally long-running development servers.

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
- Consider placing it next to the traffic lights, similar to Codex App.
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

## Test squircle corners for Rio UI

Evaluate whether the squircle/super-ellipse algorithm from the Love project fits Rio's UI corners.

- Compare it against the current rounded-rect treatment.
- Check floating sidebar, tab pills, and update pills.
- Keep the current native-feeling macOS sidebar direction unless the squircle version is clearly better.

## Run font and theme experiments

Validate candidate fonts and theme combinations for the current sidebar design.

- Test Departure Mono bold number rendering in Ghostty to see whether the glyphs come from fallback.
- Test Geist Mono with the current Cursor Dark theme and sidebar layout.
- Compare the result against Berkeley Mono before changing defaults.

## Set startup window defaults for the fork

Make Rio launch into the preferred personal-fork layout.

- Set a fixed startup window size in config so Rio launches non-maximized.
- Prefer one pane per tab instead of splits on startup.
- Keep the traffic lights visible.

## Set up fork release infrastructure

Prepare the personal fork for public builds and ongoing upstream sync.

- Keep the fork separate because tab groups, Git stats, and related sidebar work are too opinionated for upstream.
- Set up release CI with GitHub Actions or the self-hosted server.
- Build and publish platform artifacts publicly.
- Keep the fork in sync with upstream while upstream commit cadence remains manageable.
