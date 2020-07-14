all:
	echo all

setup-hooks:
	ln -f git-hooks/* .git/hooks
