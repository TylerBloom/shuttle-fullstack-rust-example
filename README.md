## TL;DR
To serve the webapp in debug mode, run `cargo shuttle run` from the `backend/` and `trunk serve` from `frontend/` after you've installed the dependecies. For release mode, you only need to run `cargo shuttle run --release` the build process takes care of everything else. When deployed on Shuttle, your app will be compiled in release mode.

## About
This repo contains a basic layout for a fullstack Rust project (both frontend and backend in Rust). This contains of a backend server, a frontend webapp that is compiled to WASM, and a shared model.

Rather than focus on any partial framework (or pair of frameworks), this template focuses on the broad strokes concepts needed for such a project to work. In other words, you can substitute your desired frontend and backend crates with no additional work. This project only seeks to show how to get two to talk to each other.

## Structure
This repo contains three subcrates: `frontend`, `backend`, and `model`. The reason for the first two subcrates is fairly self explanatory. The third crate is for all common code shared between the ends of your app. In this, very simply, example, the `model` is an empty crate, but you would want to put things like:
				- API paths
				- API data structs
				- Client code (if needed for a desktop client, for example)

The `model` and the `backend` are both in the workspace defined at the root of this repo, and the `frontend` is intentionally excluded. This helps with a more seemless build process (see the `backend` README for more details). There is also an `assets` directory. Everything in there is ignored by git, so you should not store static content in there. Instead, that directory is used during the build process for the `backend`. The build artifacts for the frontend are stored there, and then are bundled into the `backend` to avoid any fussing with static files.

## Development Flow
This project is setup to allows for different flows for development and deployment.

When deployed, all frontend artifacts are statically bound into the backend's binary. This includes the WASM module, the JS bridge code, and the HTML index. Specifically, this happens when compiled in Shuttle's envinorment or when compiled in release model.

When compiled in debug mode (i.e. during the standard dev process), the backend ignores the frontend and the frontend should be deployed seperately.
Trunk is an excellent tool for this.
With it, you can proxy all calls to the backend, hot reload the frontend on save without recompiling the backend, and easily make the frontend accessible from other devices (like mobile devices) that are on the same network.
To do this, make your working directory `frontend/` and simply run `trunk serve`.
After the frontend is compiled and served, you should see output akin to this:
```
    🏠 http://127.0.0.1:8080
    💻 http://192.168.202.218:8080
    💻 http://172.0.0.1:8080
```

This setup assumes that the backend's port is 8000, and that you want to proxy calls whose API route starts with `/api/`.
To adjust this, edit `frontend/Trunk.toml`.

## Contributing
If you have any improvements to this template, please open an issue or PR! Otherwise, feel free to using this project as the basis of any of your own projects.
