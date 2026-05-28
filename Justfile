PORT := "6942"

# run the application on the provided port

run: (run-on PORT)

run-on port:
    dx run --release --port {{ port }}

resume:
    typst compile assets/resume.typ
