all:
	go run .

serve:
	go run . -serve

clean:
	rm -rf public