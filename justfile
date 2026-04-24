test:
    R -e "devtools::load_all()"
    cd src/rust && cargo test

doc:
    R -e "pkgdown::build_site(install = FALSE)"

check:
    R -e "devtools::check()"
