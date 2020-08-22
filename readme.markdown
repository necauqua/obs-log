# obs-log
This is a very simple bridge library between the `log` crate, which is a
logging facade, and the OBS plugin logging API.
The api is very simple and just maps log calls to `blog` with appropriate
logging levels, applying formatting on the Rust side with a `format!` call.

Both `debug` and `trace` levels are mapped into `LOG_DEBUG` OBS level since
OBS has no trace level.

It links to the `obs` dynamic library, so make sure you have that installed.

As with any `log` facade, simply put a call to `obs_log::install()` or 
`obs::log::install_with_level(...)` before using log macros, e.g. in your
`obs_module_load` implementation.

Install methods can be called multiple times - next invocations will do nothing.

## License
This small piece of software is licensed under the MIT license which means that
you can do whatever you want with it as long as you include the `LICENSE` file
(which has my name written on top of it).

## Contribution
This crate is not intended to be changed much, but issues/PR's that do not
suggest/add new features (since it is feature-complete) are welcome.
