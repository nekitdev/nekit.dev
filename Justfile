PORT := "6942"

# run the application on the provided port

run: (run-on PORT)

run-on port:
    dx run --release --port {{ port }}

resume language:
    typst compile assets/resume/{{ language }}.typ
