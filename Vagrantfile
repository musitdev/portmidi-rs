# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "precise64"
  config.vm.provision "shell", inline: <<-SHELL
    sudo apt-get update
    # install portmidi
    sudo apt-get install -y libportmidi-dev
    # install curl
    sudo apt-get install -y curl
    # setup 'rustup.sh'
    sudo sh -c 'echo "curl -L https://static.rust-lang.org/rustup.sh | sudo sh" > /usr/bin/rustup.sh'
    sudo chmod +x /usr/bin/rustup.sh
    # run 'rustup.sh'
    sudo /usr/bin/rustup.sh
    # add 'vagrant' user to audio group so that they can access Midi
    sudo gpasswd -a vagrant audio
  SHELL
end
