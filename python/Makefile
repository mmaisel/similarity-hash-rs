NAME := fuzzy

env:
	python -m venv env
	env/bin/pip install -r dev-requirements.txt

build: env clean
	cargo build --release
	# cp target/release/lib$(NAME).so $(NAME)/$(NAME).so
	env/bin/pip install -e .

test: build
	env/bin/pytest -v tests
	# env/bin/tox

clean:
	rm -rf $(NAME)/*.so

clean-env:
	rm -rf env
	rm -rf target/
	rm -rf *egg-info