# rust-htmx-play

## History

* 2023-09-04 PESmit - create initial files using axum as it integrates well with tokio

## ToDo

* [] Remove tower - not used
* [] Remove serve_dir.rs, replaced with in binary static /s

## http endpoints

* /s/ has static files loaded into the binary !
* /static/ loads files dynamically from ./static/
* /hello/:name responds with :name

## Debug run

      cargo install cargo-watch
      cargo watch -x run

      # In 2nd terminal
      curl -i http://127.0.0.1:8080/
      curl -i http://127.0.0.1:8080/s/index.html
      curl -i http://127.0.0.1:8080/hello/testing
      curl -i http://127.0.0.1:8080/static/hello2.html





      