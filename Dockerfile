FROM ubuntu:latest

ARG user=dev

RUN apt-get update && \
    apt-get upgrade --yes

# safe to ignore warning...
#   debconf: delaying package configuration, since apt-utils is not installed
#   https://stackoverflow.com/questions/51023312/docker-having-issues-installing-apt-utils

RUN apt-get install --yes --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    emacs-nox \
    git \
    less \
    libssl-dev \
    pkg-config \
    unzip

# nodejs: apt-get would install nodejs v8 but we want current version
RUN curl --show-error --silent --location https://deb.nodesource.com/setup_12.x | bash - && \
    apt-get install --yes --no-install-recommends nodejs

# yarn: ubuntu:latest comes with a conflicting yarn alias, cmdtest.
RUN apt-get remove --yes cmdtest && \
    curl --show-error --silent https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add - > /dev/null 2>&1 && \
    echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list && \
    apt-get update && \
    apt-get install --yes --no-install-recommends yarn

# cleanup apt-get cruft
RUN apt-get autoremove --yes && \
    rm --recursive --force /var/lib/apt/lists/*

# create user for development
RUN useradd --create-home --shell /bin/bash ${user}

# copy github repo into dev user's home
COPY --chown=dev:dev . /home/${user}/app

# dev user for any run, cmd, or entrypoint instructions added later...
WORKDIR /home/${user}
USER ${user}

# add yarn bin to dev's path
RUN echo export PATH="$(yarn global bin):$PATH" >> .bashrc

# rustup
RUN curl --proto '=https' --tlsv1.2 -sf curl https://sh.rustup.rs | bash -s -- -y

# add cargo and rustup to dev's path at next login
RUN echo "source $HOME/.cargo/env" >> .bashrc

# rust nightly is needed to run the benchmark tests
RUN . $HOME/.cargo/env && rustup install nightly

# cargo-generate
RUN . $HOME/.cargo/env && cargo install cargo-generate

# wasmpack
RUN . $HOME/.cargo/env && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sf | bash -s -- -y

# expose react's development webserver port
EXPOSE 3000/tcp

# expose wasm development webserver port
EXPOSE 8080/tcp
