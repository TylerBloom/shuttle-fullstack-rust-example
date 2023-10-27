## TL;DR
To serve the webapp, all that you need to do is run `cargo shuttle run` from the backend sub-crate after you've installed the dependecies. That's it. The build process takes care of everything else.

## About
This repo contains a basic layout for a fullstack Rust project (both frontend and backend in Rust). This contains of a backend server, a frontend webapp that is compiled to WASM, and a shared model.

Rather than focus on any partial framework (or pair of frameworks), this template focuses on the broad strokes concepts needed for such a project to work. In other words, you can substitute your desired frontend and backend crates with no additional work. This project only seeks to show how to get two to talk to each other.

The goal of this project design is to be as simple to run as possible. When all is said and done, you only need to compile and run the backend (either locally or deployed on Shuttle) for everything to work.

## Structure
This repo contains three subcrates: `frontend`, `backend`, and `model`. The reason for the first two subcrates is fairly self explanatory. The third crate is for all common code shared between the ends of your app. In this, very simply, example, the `model` is an empty crate, but you would want to put things like:
				- API paths
				- API data structs
				- Client code (if needed for a desktop client, for example)

The `model` and the `backend` are both in the workspace defined at the root of this repo, and the `frontend` is intentionally excluded. This helps with a more seemless build process (see the `backend` README for more details). There is also an `assets` directory. Everything in there is ignored by git, so you should not store static content in there. Instead, that directory is used during the build process for the `backend`. The build artifacts for the frontend are stored there, and then are bundled into the `backend` to avoid any fussing with static files.

More in-depth explanations can be found in the READMEs of the sub-crates.

## Contributing
If you have any improvements to this template, please open an issue or PR! Otherwise, feel free to using this project as the basis of any of your own projects.
