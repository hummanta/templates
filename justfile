default:
    @just --list

generate-all:
    just generate-detector
    just generate-frontend
    just generate-backend

# The purpose of these targets is to make it easy to make changes to the templates and then
# regenerate the generated projects and view the expected changes in a git diff.

generate-detector:
    rm -rv detector-generated | true
    cargo generate --path ./detector \
        --name detector-generated \
        --define description="An example generated using the detector template" \
        --define github-username="hummanta" \
        --define language="Toy"

generate-frontend:
    rm -rv frontend-generated | true
    cargo generate --path ./frontend \
        --name frontend-generated \
        --define description="An example generated using the frontend template" \
        --define github-username="hummanta" \
        --define language="Toy"

generate-backend:
    rm -rv backend-generated | true
    cargo generate --path ./backend \
        --name backend-generated \
        --define description="An example generated using the backend template" \
        --define github-username="hummanta"
