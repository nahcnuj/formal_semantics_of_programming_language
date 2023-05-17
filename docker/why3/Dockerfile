FROM ubuntu:22.10

ARG OPAM_VERSION=2.1.3
ARG OCAML_VERSION=4.14.1
ARG CREUSOT_REV=7c8f33dcd9a7d734e95ec106b47107baa4dc1f72

RUN sed -i 's@archive.ubuntu.com@ftp.jaist.ac.jp/pub/Linux@g' /etc/apt/sources.list

RUN <<EOF
apt-get update
apt-get install -y \
    bubblewrap \
    coqide \
    curl \
    gcc \
    libcairo2-dev \
    libexpat1-dev \
    libgmp-dev \
    libgtk-3-dev \
    libgtksourceview-3.0-dev \
    make \
    patch \
    perl \
    pkg-config \
    unzip
apt-get clean
rm -r /var/lib/apt/lists/*
EOF

RUN <<EOF
curl -sL "https://github.com/ocaml/opam/releases/download/${OPAM_VERSION}/opam-${OPAM_VERSION}-x86_64-linux" -o /tmp/opam
install /tmp/opam /usr/local/bin/
rm /tmp/opam
EOF

WORKDIR /workspace

RUN <<EOF
opam init -y --disable-sandboxing
eval $(opam env)
opam switch create ${OCAML_VERSION}
EOF

# eval $(opam env)
ENV OPAM_SWITCH_PREFIX=/root/.opam/${OCAML_VERSION}
ENV CAML_LD_LIBRARY_PATH=${OPAM_SWITCH_PREFIX}/lib/stublibs:${OPAM_SWITCH_PREFIX}/lib/ocaml/stublibs:${OPAM_SWITCH_PREFIX}/lib/ocaml
ENV OCAML_TOPLEVEL_PATH=${OPAM_SWITCH_PREFIX}/lib/toplevel
ENV PKG_CONFIG_PATH=${OPAM_SWITCH_PREFIX}/lib/pkgconfig
ENV MANPATH=${OPAM_SWITCH_PREFIX}/man
ENV PATH=${OPAM_SWITCH_PREFIX}/bin:$PATH

# install Why3
RUN <<EOF
opam install -y \
    lablgtk3 \
    lablgtk3-sourceview3 \
    ocamlgraph \
    why3 \
    why3-ide \
    why3-coq
why3 config detect
EOF

# copy Creusot preludes
RUN <<EOF
cd /tmp/
curl -sL "https://github.com/xldenis/creusot/archive/${CREUSOT_REV}.zip" -o /tmp/creusot.zip
unzip /tmp/creusot.zip
cp -a /tmp/creusot-${CREUSOT_REV}/prelude /workspace/
rm -rf /tmp/creusot.zip /tmp/creusot
EOF

ENTRYPOINT [ "why3", "-L", "/workspace/prelude" ]
