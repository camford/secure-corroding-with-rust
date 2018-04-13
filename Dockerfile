FROM fedora:27
ENV RUSTUP_HOME /opt/rust
ENV CARGO_HOME /opt/rust
ENV PATH="${PATH}:/opt/rust/bin"
RUN dnf install -y gcc clang make libcxx-devel golang golang-bin patch \
 && curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain 1.25.0 \
 && mkdir /opt/rust/registry \
 && chmod 777 /opt/rust/registry \
 && rustup install nightly \
 && cargo +nightly install clippy
