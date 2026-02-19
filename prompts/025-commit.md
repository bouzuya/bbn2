Consolidate build_html and build_json modules into the build module

Move the HTML and JSON generation logic from the top-level
command/build_html.rs and command/build_json.rs into submodules
command/build/html.rs and command/build/json.rs. Convert command/build.rs
into command/build/mod.rs and update it to reference the new submodules
directly. Remove the now-redundant build_html and build_json module
declarations from command.rs.
