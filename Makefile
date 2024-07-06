dev:
	cargo watch -c -w "src" -x "lrun serve"

tw-build:
	npx tailwindcss -i styles/tailwind.css -o static/dist/css/style.css

tw-dev:
	npx tailwindcss -i styles/tailwind.css -o static/dist/css/style.css -w

format:
	rustywind --write .

clean:
	rm -f ./static/dist/css/*.gz
