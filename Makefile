all:
	echo all

clean-git-hooks:
	rm .git/hooks/*

setup-hooks: clean-git-hooks
	ln -f git-hooks/* .git/hooks

clean-a17:
	sudo rm -rf examples/a17/root/file.sh
