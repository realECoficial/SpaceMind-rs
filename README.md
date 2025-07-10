# Development

hola, disclaimer, este mini tutorial es solo para Arch linux 86_64.
si tienes windows u o mac u otra distribucion [clickea aca.](https://dioxuslabs.com/learn/0.6/getting_started/#platform-specific-dependencies)

Antes de siquiera hacer algo, necesitaran estas dependencias para el OS:
```
sudo pacman -Syu
sudo pacman -S --needed \
  webkit2gtk-4.1 \
  base-devel \
  curl \
  wget \
  file \
  openssl \
  appmenu-gtk-module \
  libappindicator-gtk3 \
  librsvg \
  xdotool
```

Para usarla tienes que tener la ultima version de rustup junto con cargo.
(`rustup --version` y `cargo --version`).

tiene algunos crates para hacer fetching:

`cargo add reqwest --features json`
`cargo add serde --features derive`

Ahora para tener los "target" solo tienen que usar: 

`dx serve --platform web`


WORK IN PROCESS, traten esto solo como una prueba de la tecnologia. El producto mostrado aca no tiene nada que ver con el que sera el final.

