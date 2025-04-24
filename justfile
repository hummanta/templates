default:
    @just --list

generate-all:
    just generate-detector

# The purpose of these targets is to make it easy to make changes to the templates and then
# regenerate the generated projects and view the expected changes in a git diff.

generate-detector:
    rm -rv detector-generated | true
    cargo generate --path ./detector \
        --name detector-generated \
        --define description="An example generated using the detector template" \
        --define github-username="hummanta" \
        --define language="Rust"
