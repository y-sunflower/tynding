test:
    R -e "devtools::load_all()"
    cd src/rust && cargo test
