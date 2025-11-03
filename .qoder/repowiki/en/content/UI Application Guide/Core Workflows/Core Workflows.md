# Core Workflows

<cite>
**Referenced Files in This Document**   
- [drop_zone.rs](file://abogen-ui/crates/ui/components/drop_zone.rs)
- [state.rs](file://abogen-ui/crates/ui/state.rs)
- [slider.rs](file://abogen-ui/crates/ui/components/slider.rs)
- [combo.rs](file://abogen-ui/crates/ui/components/combo.rs)
- [queue_modal.rs](file://abogen-ui/crates/ui/components/queue_modal.rs)
- [progress_bar.rs](file://abogen-ui/crates/ui/components/progress_bar.rs)
- [log_panel.rs](file://abogen-ui/crates/ui/components/log_panel.rs)
- [README.md](file://abogen-ui/README.md)
</cite>

## Table of Contents
1. [Main Screen Workflow](#main-screen-workflow)
2. [Configuration Process](#configuration-process)
3. [Queue Management](#queue-management)
4. [Processing Mode Transition](#processing-mode-transition)
5. [Processing Screen Workflow](#processing-screen-workflow)
6. [Error Handling Scenarios](#error-handling-scenarios)

## Main Screen Workflow

The Main Screen serves as the primary user interface for initiating file processing in the Abogen UI application. Users can select files through two interaction patterns: drag-and-drop or click-based file browsing. The `DropZone` component, implemented in `drop_zone.rs`, provides a visually distinct panel with neon styling that accepts file inputs of types `.txt`, `.epub`, and `.pdf`. When a user drags a file over the drop zone, the component provides visual feedback by changing its border color to cyan and displaying a glow effect, indicating readiness to receive the file.

For click-based interaction, the component renders a hidden file input element that is triggered when the user clicks on the labeled area. This activates the system's native file picker dialog, allowing users to navigate their file system and select an appropriate file. Upon file selection or drop, the application extracts metadata including the file name, path, and size, creating a `FileInfo` struct that is stored in the global `AppState.selected_file` signal. This reactive state update triggers UI re-rendering to display the selected filename and formatted file size below the drop zone, providing immediate visual confirmation of successful file selection.

The implementation handles both desktop and web platforms through conditional compilation. On desktop targets, it uses native file system APIs to access file metadata, while on web targets (WASM), it leverages browser file APIs to extract file information without requiring server-side processing. This cross-platform compatibility ensures consistent user experience across different deployment targets.

**Section sources**
- [drop_zone.rs](file://abogen-ui/crates/ui/components/drop_zone.rs#L1-L235)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L4-L15)

## Configuration Process

After file selection, users configure processing parameters through several interactive components that update the application state in real time. The configuration interface includes three primary controls: a speed adjustment slider, a voice selection combobox, and output format options.

The speed adjustment is handled by the `Slider` component from `slider.rs`, which allows users to set playback speed between 0.5x and 2.0x. As users manipulate the slider thumb, the component immediately updates the `AppState.speed` signal with the new value, displaying the current speed as a formatted string (e.g., "1.25x") in a monospace font for precise readability. This real-time feedback enables users to fine-tune the audio output characteristics according to their preferences.

Voice selection is managed by the `Combo` component in `combo.rs`, which implements a searchable dropdown interface for voice options. The component displays both the voice name and its associated language, allowing users to identify appropriate voices for their content. When users begin typing in the input field, the component filters available voices based on name, ID, or language, making it easy to locate specific voices in large catalogs. Selecting a voice updates the `AppState.voice` signal, which will be used during processing.

Additional configuration options include subtitle generation mode (`SubtitleMode` enum), voice output format (`VoiceFormat` enum), subtitle format (`SubtitleFormat` enum), newline replacement behavior, save location (`SaveLocation` enum), and GPU acceleration preference. These settings are stored in corresponding signals within `AppState`, creating a comprehensive configuration profile that will be applied when the file is processed.

**Section sources**
- [slider.rs](file://abogen-ui/crates/ui/components/slider.rs#L1-L53)
- [combo.rs](file://abogen-ui/crates/ui/components/combo.rs#L1-L146)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L17-L106)

## Queue Management

The queue management system allows users to organize multiple files for batch processing. After configuring settings for a selected file, users can click the "ADD TO QUEUE" button to create a `QueuedItem` that captures the current file and all configuration options. This item is then appended to the `AppState.queue` signal, which maintains a list of processing jobs.

The `QueueModal` component from `queue_modal.rs` provides a dedicated interface for managing the processing queue. When users click "MANAGE QUEUE", a modal dialog displays all queued items with their associated metadata, including filename, selected voice, speed setting, and output formats. Each queue entry includes controls to reorder items (move up/down buttons) or remove individual entries, allowing users to refine their processing order.

The modal interface supports keyboard navigation and includes visual indicators for the current queue status. When the queue is empty, it displays a descriptive message prompting users to add items. For non-empty queues, it presents a scrollable list of items with consistent styling that matches the application's cyberpunk aesthetic. Users can also clear all entries at once through a dedicated "CLEAR QUEUE" operation, which empties the `AppState.queue` signal.

This queue system enables users to prepare multiple processing jobs with different configurations, then execute them sequentially without further interaction. The implementation ensures that each queued item contains a complete snapshot of its configuration at the time of addition, preventing unintended changes if global settings are modified later.

**Section sources**
- [queue_modal.rs](file://abogen-ui/crates/ui/components/queue_modal.rs#L1-L195)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L17-L25)

## Processing Mode Transition

When users click the "START" button with a non-empty queue, the application transitions from configuration mode to processing mode by mutating key state variables. This transition is governed by the `AppState.is_processing` signal, which changes from `false` to `true`, triggering a UI re-render that switches from the Main Screen to the Processing Screen.

The transition process begins by validating the queue contents and preparing processing parameters for the first item. The application creates a `QueuedItem` struct containing the file information and all configuration settings, which will be passed to the processing pipeline. During this phase, the UI becomes read-only, preventing users from modifying settings or queue contents while processing is active.

Concurrently, the `AppState.progress` signal is reset to 0, and the `AppState.logs` signal is initialized with an empty vector to capture processing output. A `cancel_token` signal is also prepared, which can be used to signal cancellation to long-running operations if the user chooses to abort processing.

This state mutation pattern ensures that the UI accurately reflects the application's operational mode. The transition is immediate and seamless, with no intermediate states that could confuse users about whether processing has begun. The use of reactive signals guarantees that all UI components update consistently when the processing state changes.

**Section sources**
- [state.rs](file://abogen-ui/crates/ui/state.rs#L108-L135)

## Processing Screen Workflow

The Processing Screen provides real-time feedback during file processing through two primary components: a log panel and a progress bar. These components are bound to reactive signals in the `AppState` that are updated by the processing pipeline as operations progress.

The `LogPanel` component from `log_panel.rs` displays processing output by iterating over the `AppState.logs` signal, which contains a vector of `LogEntry` structs. Each log entry includes a message string and a `LogLevel` enum value that determines its visual styling. The component renders logs in a monospace font within a scrollable panel, with color coding based on log level: cyan for informational messages, amber for notices, and red for errors. This immediate visual feedback helps users quickly identify the processing status and any potential issues.

The `ProgressBar` component from `progress_bar.rs` visualizes processing completion through a horizontal bar that fills from left to right as the `AppState.progress` signal updates. The progress value is represented as an integer percentage (0-100), with the current percentage displayed as text within the bar itself. This component provides a clear, intuitive representation of processing completion that requires minimal cognitive effort to interpret.

As processing proceeds, background tasks update these signals with current status information. The UI automatically re-renders whenever these signals change, creating a dynamic, real-time view of the processing pipeline. When processing completes (either successfully or with errors), the `is_processing` signal is set back to `false`, triggering a return to the Main Screen where users can begin a new processing cycle.

**Section sources**
- [log_panel.rs](file://abogen-ui/crates/ui/components/log_panel.rs#L1-L44)
- [progress_bar.rs](file://abogen-ui/crates/ui/components/progress_bar.rs#L1-L24)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L125-L135)

## Error Handling Scenarios

The application implements comprehensive error handling for common failure scenarios during file processing. When a file read operation fails—such as when a selected file is moved or deleted after selection—the system captures the error and updates the `AppState.logs` signal with an error-level log entry describing the issue. This prevents the application from crashing while still informing users of the problem.

Processing interruptions, such as network failures during video generation or TTS service unavailability, are handled through the `cancel_token` mechanism. Long-running operations periodically check the `AppState.cancel_token` signal, allowing them to terminate gracefully when processing is canceled. This prevents resource leaks and ensures the application remains responsive even when external services fail to respond.

For validation errors, such as unsupported file formats or invalid configuration combinations, the application performs checks before initiating processing. These preemptive validations provide immediate feedback, preventing users from starting processes that are destined to fail. The error messages are added to the logs with appropriate severity levels, and the processing workflow is halted until the user corrects the issues.

All error handling follows a consistent pattern: capture the error condition, update the reactive state with descriptive information, and ensure the UI reflects the current status. This approach maintains application stability while providing users with the information they need to resolve issues and continue their workflow.

**Section sources**
- [drop_zone.rs](file://abogen-ui/crates/ui/components/drop_zone.rs#L65-L85)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L137-L145)
- [log_panel.rs](file://abogen-ui/crates/ui/components/log_panel.rs#L1-L44)