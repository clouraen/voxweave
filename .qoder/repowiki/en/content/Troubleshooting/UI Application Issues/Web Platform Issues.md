# Web Platform Issues

<cite>
**Referenced Files in This Document**   
- [Trunk.toml](file://abogen-ui/apps/web/Trunk.toml)
- [main.rs](file://abogen-ui/apps/web/src/main.rs)
- [lib.rs](file://abogen-ui/crates/ui/lib.rs)
- [state.rs](file://abogen-ui/crates/ui/state.rs)
- [theme.rs](file://abogen-ui/crates/ui/theme.rs)
- [index.html](file://abogen-ui/apps/web/index.html)
</cite>

## Table of Contents
1. [Introduction](#introduction)
2. [Trunk.toml Configuration Issues](#trunktoml-configuration-issues)
3. [WASM Build Failures and Asset Loading](#wasm-build-failures-and-asset-loading)
4. [Hydration Mismatches and Routing Problems](#hydration-mismatches-and-routing-problems)
5. [Signal Propagation in Web Components](#signal-propagation-in-web-components)
6. [Browser Console Debugging and JavaScript Interop](#browser-console-debugging-and-javascript-interop)
7. [Performance Bottlenecks in Web Renderer](#performance-bottlenecks-in-web-renderer)
8. [CSS Injection and Theme Application](#css-injection-and-theme-application)
9. [Resolving E0716 Temporary Value Errors](#resolving-e0716-temporary-value-errors)

## Introduction
This document addresses common web-specific issues encountered in the Dioxus-based UI for the abogen project. The application targets multiple platforms including web, desktop, and mobile, with the web platform presenting unique challenges related to configuration, compilation, rendering, and state management. The UI implements a cyberpunk aesthetic with Rajdhani font and neon styling, built using Dioxus 0.7 framework. Key components include the ProcessingScreen for handling asynchronous operations and lib.rs for state management. This guide provides troubleshooting solutions for the most frequent web platform issues.

## Trunk.toml Configuration Issues
Configuration errors in Trunk.toml can prevent proper web application compilation and serving. The current configuration specifies the build target as index.html with output to the dist directory, serves on port 8080, and watches relevant source directories.

Common issues include incorrect file paths, missing watch directories, or invalid serve configurations. The provided Trunk.toml correctly configures the build process with appropriate watch targets including the web source and shared UI components. Ensure that the dist directory is properly specified and that the serve port is available.

**Section sources**
- [Trunk.toml](file://abogen-ui/apps/web/Trunk.toml#L1-L12)

## WASM Build Failures and Asset Loading
WASM build failures often stem from dependency conflicts or incorrect feature flags. The project uses dioxus-web 0.7.0 with dependencies including wasm-bindgen, web-sys, and gloo-timers. Asset loading issues can occur when external resources like fonts are not properly referenced.

The application successfully loads the Rajdhani font through Google Fonts, as evidenced by the @import statement in the theme CSS. Ensure that the index.html file includes proper meta tags and that Trunk can resolve all asset paths. The build process should compile Rust code to WASM and bundle it with the JavaScript runtime.

**Section sources**
- [Trunk.toml](file://abogen-ui/apps/web/Trunk.toml#L1-L12)
- [index.html](file://abogen-ui/apps/web/index.html#L1-L20)
- [theme.rs](file://abogen-ui/crates/ui/theme.rs#L1-L209)

## Hydration Mismatches and Routing Problems
Hydration mismatches occur when server-rendered HTML does not match client-side rendered content. In this Dioxus application, the App component manages routing between Main and Processing screens using a Screen enum and signal-based state.

The InnerApp component handles screen transitions by matching the current_screen signal value. Ensure that initial state is consistent between server and client rendering. The routing logic in lib.rs correctly implements screen switching through signal mutations, with proper handling of processing state and queue management.

**Section sources**
- [lib.rs](file://abogen-ui/crates/ui/lib.rs#L1-L640)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L1-L256)

## Signal Propagation in Web Components
Signal propagation issues can disrupt state management across components. The application uses Dioxus signals for state management, with AppState containing signals for various UI states. The pattern of cloning signals before mutation is correctly implemented.

Signal mutations follow the established pattern of creating a mutable copy of the signal and writing to it, which prevents temporary value lifetime issues. Components like MainScreen and ProcessingScreen receive state via props and use EventHandlers for callback propagation. Ensure that signal dependencies are properly tracked and that updates trigger appropriate re-renders.

**Section sources**
- [lib.rs](file://abogen-ui/crates/ui/lib.rs#L1-L640)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L1-L256)

## Browser Console Debugging and JavaScript Interop
Debugging web applications requires effective use of browser developer tools. The application uses standard console logging through the log crate, with messages appearing in the browser console. JavaScript interop is handled through wasm-bindgen and web-sys crates.

Common interop issues include type mismatches between Rust and JavaScript, incorrect function signatures, or timing issues with asynchronous operations. The build configuration includes necessary dependencies for JavaScript interop, and the application can safely call into JavaScript when needed. Use browser breakpoints and console inspection to diagnose interop problems.

**Section sources**
- [Cargo.lock](file://abogen-ui/Cargo.lock#L1397-L1453)
- [Cargo.lock](file://abogen-ui/Cargo.lock#L1469-L1530)

## Performance Bottlenecks in Web Renderer
Performance issues in the web renderer can manifest as slow rendering, janky animations, or high memory usage. The application uses efficient Dioxus rendering patterns, with conditional rendering based on signal values.

The ProcessingScreen component efficiently updates the progress bar and log panel without unnecessary re-renders. Long-running operations are spawned in async tasks to prevent blocking the main thread. Monitor performance using browser devtools to identify expensive re-renders or memory leaks. Consider optimizing large list rendering or complex computations.

**Section sources**
- [lib.rs](file://abogen-ui/crates/ui/lib.rs#L1-L640)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L1-L256)

## CSS Injection and Theme Application
CSS injection issues can prevent proper styling application. The application injects global styles through a style tag in the App component, using the get_theme_css function from theme.rs.

The cyberpunk theme is successfully applied with neon colors, Rajdhani font, and custom styling for components. The CSS includes proper font loading, variable definitions, and responsive design. Ensure that the style tag is placed early in the component tree and that there are no CSS conflicts with browser defaults.

**Section sources**
- [lib.rs](file://abogen-ui/crates/ui/lib.rs#L1-L640)
- [theme.rs](file://abogen-ui/crates/ui/theme.rs#L1-L209)

## Resolving E0716 Temporary Value Errors
E0716 errors occur when a temporary value is dropped while still borrowed. In the context of Dioxus signals, this commonly happens when attempting to mutate a signal without properly handling its lifetime.

The solution, as documented in STATUS.md, is to follow the signal mutation pattern:
```rust
let mut signal_copy = original_signal;
*signal_copy.write() = new_value;
```

This works because signals are Copy types in Dioxus, allowing them to be cloned before mutation. This pattern prevents the temporary value from being dropped prematurely. Apply this pattern consistently throughout the codebase when mutating signals.

**Section sources**
- [STATUS.md](file://abogen-ui/STATUS.md#L196-L206)
- [lib.rs](file://abogen-ui/crates/ui/lib.rs#L1-L640)
- [state.rs](file://abogen-ui/crates/ui/state.rs#L1-L256)