dev:
	cargo watch -w "src" -x "run serve"

# compile tailwind css
tw:
	npx tailwindcss -i styles/tailwind.css -o static/dist/css/style.css

# compile tailwind css style in watch mode
tw-dev:
	npx tailwindcss -i styles/tailwind.css -o static/dist/css/style.css -w

format:
	rustywind --write .

clean:
	rm -f ./static/dist/css/*.gz
