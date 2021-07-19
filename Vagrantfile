# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure("2") do |config|
  config.vm.box = "ubuntu/hirsute64"

  config.vm.provision "shell", inline: "apt-get update && apt-get install -y build-essential clang libelf-dev libc6-dev-i386 unzip"
  config.vm.provision "shell", path: "libbpf.sh"
  config.vm.provision "shell", inline: "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y", privileged: false
  config.vm.provision "shell", inline: "cargo install libbpf-cargo", privileged: false

end
